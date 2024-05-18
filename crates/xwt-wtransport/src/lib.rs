#![cfg_attr(
    not(target_family = "wasm"),
    doc = "The [`wtransport`]-powered implementation of [`xwt_core`]."
)]
#![cfg_attr(
    target_family = "wasm",
    doc = "The `wtransport`-powered implementation of `xwt_core`."
)]
#![cfg(not(target_family = "wasm"))]

mod types;

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
    type Error = wtransport::error::StreamReadError;

    async fn read(&mut self, buf: &mut [u8]) -> Result<Option<usize>, Self::Error> {
        self.0.read(buf).await
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
    type Error = wtransport::error::StreamWriteError;

    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.0.write(buf).await
    }
}

impl xwt_core::stream::WriteChunk<xwt_core::stream::chunk::U8> for SendStream {
    type Error = wtransport::error::StreamWriteError;

    async fn write_chunk<'a>(&'a mut self, buf: &'a [u8]) -> Result<(), Self::Error> {
        self.0.write_all(buf).await
    }
}

impl xwt_core::stream::WriteAbort for SendStream {
    type ErrorCode = StreamErrorCode;
    type Error = std::convert::Infallible;

    async fn abort(self, error_code: Self::ErrorCode) -> Result<(), Self::Error> {
        let code = error_codes::to_http(error_code.0).try_into().unwrap();
        self.0.reset(code);
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

    async fn aborted(self) -> Result<Self::ErrorCode, Self::Error> {
        match self.0.stopped().await {
            wtransport::error::StreamWriteError::Stopped(val) => {
                let code = error_codes::from_http(val.into_inner())
                    .map_err(WriteAbortedError::ErrorCodeConversion)?;
                Ok(StreamErrorCode(code))
            }
            err => Err(WriteAbortedError::StreamWrite(err)),
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

/// Conversions of the WebTransport error codes from and to the HTTP3 error
/// codes.
///
/// We need this because [`wtransport`] does not convert error codes at all.
pub mod error_codes {
    /// Lower end of the WebTransport HTTP codes.
    const FIRST: u64 = 0x52e4a40fa8db;
    /// Higher end of the WebTransport HTTP codes.
    const LAST: u64 = 0x52e5ac983162;

    /// Convert the WebTransport code value to HTTP3 code.
    pub fn to_http(n: u32) -> u64 {
        if n == 0 {
            return 0;
        }

        let n: u64 = n.into();
        FIRST + n + (n / 0x1e)
    }

    /// An error that occurs when converting the HTTP error code to
    /// WebTransport error code.
    #[derive(Debug, thiserror::Error)]
    #[error("unable to convert HTTP error code to WebTransport error code: {0}")]
    pub struct FromHttpError(u64);

    /// Convert the HTTP3 code value to WebTransport code.
    pub fn from_http(h: u64) -> Result<u32, FromHttpError> {
        if h == 0 {
            return Ok(0);
        }

        if !((FIRST..=LAST).contains(&h) && (h - 0x21) % 0x1f != 0) {
            return Err(FromHttpError(h));
        }
        let shifted = h - FIRST;
        let val = shifted - (shifted / 0x1f);
        Ok(val.try_into().unwrap())
    }
}
