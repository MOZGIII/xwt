#![cfg_attr(
    not(target_family = "wasm"),
    doc = "The [`wtransport`]-powered implementation of [`xwt_core`]."
)]
#![cfg_attr(
    target_family = "wasm",
    doc = "The `wtransport`-powered implementation of `xwt_core`."
)]
#![cfg(not(target_family = "wasm"))]

mod as_error_code;
pub mod error_codes;
mod types;

use std::num::NonZeroUsize;

pub use wtransport;
pub use xwt_core as core;

pub use self::types::*;

impl xwt_core::endpoint::Connect for Endpoint<wtransport::endpoint::endpoint_side::Client> {
    type Connecting = xwt_core::utils::dummy::Connecting<Connection>;
    type Error = wtransport::error::ConnectingError;

    async fn connect(&self, url: &str) -> Result<Self::Connecting, Self::Error> {
        let connecting = self.0.connect(url).await.map(Connection)?;
        Ok(xwt_core::utils::dummy::Connecting(connecting))
    }
}

impl xwt_core::endpoint::Accept for Endpoint<wtransport::endpoint::endpoint_side::Server> {
    type Accepting = IncomingSession;
    type Error = std::convert::Infallible;

    async fn accept(&self) -> Result<Option<Self::Accepting>, Self::Error> {
        let incoming_session = self.0.accept().await;
        let incoming_session = IncomingSession(incoming_session);
        Ok(Some(incoming_session))
    }
}

impl xwt_core::endpoint::accept::Accepting for IncomingSession {
    type Request = SessionRequest;
    type Error = wtransport::error::ConnectionError;

    async fn wait_accept(self) -> Result<Self::Request, Self::Error> {
        self.0.await.map(SessionRequest)
    }
}

impl xwt_core::endpoint::accept::Request for SessionRequest {
    type Session = Connection;
    type OkError = wtransport::error::ConnectionError;
    type CloseError = std::convert::Infallible;

    async fn ok(self) -> Result<Self::Session, Self::OkError> {
        self.0.accept().await.map(Connection)
    }

    async fn close(self, status: u16) -> Result<(), Self::CloseError> {
        debug_assert!(
            status == 404,
            "wtransport driver only supports closing requests with 404 status code"
        );
        self.0.not_found().await;
        Ok(())
    }
}

impl xwt_core::session::stream::SendSpec for Connection {
    type SendStream = SendStream;
}

impl xwt_core::session::stream::RecvSpec for Connection {
    type RecvStream = RecvStream;
}

impl tokio::io::AsyncWrite for SendStream {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        self.project().0.poll_write(cx, buf)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        self.project().0.poll_flush(cx)
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        self.project().0.poll_shutdown(cx)
    }
}

impl tokio::io::AsyncRead for RecvStream {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        self.project().0.poll_read(cx, buf)
    }
}

/// Take a pair of stream ends and wrap into our newtypes.
fn map_streams(
    streams: (wtransport::SendStream, wtransport::RecvStream),
) -> (SendStream, RecvStream) {
    let (send, recv) = streams;
    (SendStream(send), RecvStream(recv))
}

impl xwt_core::session::stream::OpeningBi for OpeningBiStream {
    type Streams = Connection;
    type Error = wtransport::error::StreamOpeningError;

    async fn wait_bi(
        self,
    ) -> Result<xwt_core::session::stream::TupleFor<Self::Streams>, Self::Error> {
        self.0.await.map(map_streams)
    }
}

impl xwt_core::session::stream::OpenBi for Connection {
    type Opening = OpeningBiStream;
    type Error = wtransport::error::ConnectionError;

    async fn open_bi(&self) -> Result<Self::Opening, Self::Error> {
        self.0.open_bi().await.map(OpeningBiStream)
    }
}

impl xwt_core::session::stream::AcceptBi for Connection {
    type Error = wtransport::error::ConnectionError;

    async fn accept_bi(&self) -> Result<xwt_core::session::stream::TupleFor<Self>, Self::Error> {
        self.0.accept_bi().await.map(map_streams)
    }
}

impl xwt_core::session::stream::OpeningUni for OpeningUniStream {
    type Streams = Connection;
    type Error = wtransport::error::StreamOpeningError;

    async fn wait_uni(
        self,
    ) -> Result<<Self::Streams as xwt_core::session::stream::SendSpec>::SendStream, Self::Error>
    {
        self.0.await.map(SendStream)
    }
}

impl xwt_core::session::stream::OpenUni for Connection {
    type Opening = OpeningUniStream;
    type Error = wtransport::error::ConnectionError;

    async fn open_uni(&self) -> Result<Self::Opening, Self::Error> {
        self.0.open_uni().await.map(OpeningUniStream)
    }
}

impl xwt_core::session::stream::AcceptUni for Connection {
    type Error = wtransport::error::ConnectionError;

    async fn accept_uni(&self) -> Result<Self::RecvStream, Self::Error> {
        self.0.accept_uni().await.map(RecvStream)
    }
}

impl xwt_core::stream::Read for RecvStream {
    type ErrorCode = StreamErrorCode;
    type Error = StreamReadError;

    async fn read(&mut self, buf: &mut [u8]) -> Result<NonZeroUsize, Self::Error> {
        match self.0.read(buf).await {
            Ok(None) => Err(StreamReadError::Closed),
            Ok(Some(val)) => match NonZeroUsize::new(val) {
                None => unreachable!(
                    "reading zero bytes from a stream that is not closed should not be possible"
                ),
                Some(val) => Ok(val),
            },
            Err(error) => Err(StreamReadError::Read(error)),
        }
    }
}

impl xwt_core::stream::ReadAbort for RecvStream {
    type ErrorCode = StreamErrorCode;
    type Error = std::convert::Infallible;

    async fn abort(self, error_code: Self::ErrorCode) -> Result<(), Self::Error> {
        let code = error_codes::to_http(error_code.0).try_into().unwrap();
        self.0.stop(code);
        Ok(())
    }
}

impl xwt_core::stream::Write for SendStream {
    type ErrorCode = StreamErrorCode;
    type Error = StreamWriteError;

    async fn write(&mut self, buf: &[u8]) -> Result<NonZeroUsize, Self::Error> {
        if buf.is_empty() {
            return Err(StreamWriteError::ZeroSizeWriteBuffer);
        }

        let len = self.0.write(buf).await.map_err(StreamWriteError::Write)?;

        let Some(len) = NonZeroUsize::new(len) else {
            unreachable!("writing zero bytes into a stream should've lead to a closed error");
        };

        Ok(len)
    }
}

impl xwt_core::stream::WriteChunk<xwt_core::stream::chunk::U8> for SendStream {
    type Error = wtransport::error::StreamWriteError;

    async fn write_chunk<'a>(&'a mut self, buf: &'a [u8]) -> Result<(), Self::Error> {
        self.0.write_all(buf).await
    }
}

impl xwt_core::session::datagram::MaxSize for Connection {
    fn max_datagram_size(&self) -> Option<usize> {
        self.0.max_datagram_size()
    }
}

impl xwt_core::stream::WriteAbort for SendStream {
    type ErrorCode = StreamErrorCode;
    type Error = wtransport::error::ClosedStream;

    async fn abort(mut self, error_code: Self::ErrorCode) -> Result<(), Self::Error> {
        let code = error_codes::to_http(error_code.0).try_into().unwrap();
        self.0.reset(code)?;
        Ok(())
    }
}

/// An error that can occur while waiting for the write stream being aborted.
#[derive(Debug, thiserror::Error)]
pub enum WriteAbortedError {
    /// An unexpected stream write error has occurred.
    #[error("stream write: {0}")]
    StreamWrite(wtransport::error::StreamWriteError),
    /// An error code failed to convert.
    #[error("error code conversion: {0}")]
    ErrorCodeConversion(error_codes::FromHttpError),
}

impl xwt_core::stream::WriteAborted for SendStream {
    type ErrorCode = StreamErrorCode;
    type Error = WriteAbortedError;

    async fn aborted(mut self) -> Result<Self::ErrorCode, Self::Error> {
        match self.0.quic_stream_mut().stopped().await {
            Ok(Some(error_code)) => {
                let code = error_codes::from_http(error_code.into_inner())
                    .map_err(WriteAbortedError::ErrorCodeConversion)?;
                Ok(code.into())
            }
            Ok(None) => Ok(0.into()),
            Err(wtransport::quinn::StoppedError::ConnectionLost(_)) => Err(
                WriteAbortedError::StreamWrite(wtransport::error::StreamWriteError::NotConnected),
            ),
            Err(wtransport::quinn::StoppedError::ZeroRttRejected) => Err(
                WriteAbortedError::StreamWrite(wtransport::error::StreamWriteError::QuicProto),
            ),
        }
    }
}

impl xwt_core::stream::Finish for SendStream {
    type Error = wtransport::error::StreamWriteError;

    async fn finish(mut self) -> Result<(), Self::Error> {
        self.0.finish().await
    }
}

impl xwt_core::session::datagram::Receive for Connection {
    type Datagram = Datagram;
    type Error = wtransport::error::ConnectionError;

    async fn receive_datagram(&self) -> Result<Self::Datagram, Self::Error> {
        self.0.receive_datagram().await.map(Datagram)
    }
}

impl AsRef<[u8]> for Datagram {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl xwt_core::session::datagram::ReceiveInto for Connection {
    type Error = wtransport::error::ConnectionError;

    async fn receive_datagram_into(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let datagram = self.0.receive_datagram().await?;
        let len = datagram.len();
        buf[..len].copy_from_slice(&datagram);
        Ok(len)
    }
}

impl xwt_core::session::datagram::Send for Connection {
    type Error = wtransport::error::SendDatagramError;

    async fn send_datagram<D>(&self, payload: D) -> Result<(), Self::Error>
    where
        D: Send + AsRef<[u8]>,
    {
        self.0.send_datagram(payload)
    }
}
