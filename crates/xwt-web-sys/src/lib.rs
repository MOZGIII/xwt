#![cfg_attr(
    target_family = "wasm",
    doc = "The [`web_wt_sys`]-powered implementation of [`xwt_core`]."
)]
#![cfg_attr(
    not(target_family = "wasm"),
    doc = "The `web_wt_sys`-powered implementation of `xwt_core`."
)]
#![cfg(target_family = "wasm")]

use std::rc::Rc;

use wasm_bindgen::JsError;

mod error;
mod options;

pub use web_sys;
pub use web_wt_sys;
pub use xwt_core as core;

pub use {error::*, options::*};

/// An endpoint for the xwt.
///
/// Internally holds the connection options and can create
/// a new `WebTransport` object on the "web" side on a connection request.
#[derive(Debug, Clone, Default)]
pub struct Endpoint {
    /// The options to use to create the `WebTransport`s.
    pub options: web_wt_sys::WebTransportOptions,
}

impl xwt_core::endpoint::Connect for Endpoint {
    type Error = Error;
    type Connecting = Connecting;

    async fn connect(&self, url: &str) -> Result<Self::Connecting, Self::Error> {
        let transport = web_wt_sys::WebTransport::new_with_options(url, &self.options)?;
        Ok(Connecting { transport })
    }
}

/// Connecting represents the transient connection state when
/// the [`web_wt_sys::WebTransport`] has been created but is not ready yet.
#[derive(Debug)]
pub struct Connecting {
    /// The WebTransport instance.
    pub transport: web_wt_sys::WebTransport,
}

impl xwt_core::endpoint::connect::Connecting for Connecting {
    type Session = Session;
    type Error = Error;

    async fn wait_connect(self) -> Result<Self::Session, Self::Error> {
        let Connecting { transport } = self;
        transport.ready().await?;

        let datagram_read_buffer_size = 65536; // 65k buffers as per spec recommendation

        let datagrams = transport.datagrams();
        let datagram_readable_stream_reader =
            web_sys_stream_utils::get_reader_byob(datagrams.readable());
        let datagram_writable_stream_writer =
            web_sys_stream_utils::get_writer(datagrams.writable());

        let datagram_read_buffer = js_sys::ArrayBuffer::new(datagram_read_buffer_size);
        let datagram_read_buffer = tokio::sync::Mutex::new(Some(datagram_read_buffer));

        let session = Session {
            transport: Rc::new(transport),
            datagram_readable_stream_reader,
            datagram_writable_stream_writer,
            datagram_read_buffer_size,
            datagram_read_buffer,
        };
        Ok(session)
    }
}

/// Session holds the [`web_wt_sys::WebTransport`] and is responsible for
/// providing access to the Web API of WebTransport in a way that is portable.
/// It also holds handles to the datagram reader and writer, as well as
/// the datagram reader state.
#[derive(Debug)]
pub struct Session {
    /// The WebTransport instance.
    pub transport: Rc<web_wt_sys::WebTransport>,
    /// The datagram reader.
    pub datagram_readable_stream_reader: web_sys::ReadableStreamByobReader,
    /// The datagram writer.
    pub datagram_writable_stream_writer: web_sys::WritableStreamDefaultWriter,
    /// The desired size of the datagram read buffer.
    /// Used to allocate the datagram read buffer in case it gets lost.
    pub datagram_read_buffer_size: u32,
    /// The datagram read internal buffer.
    pub datagram_read_buffer: tokio::sync::Mutex<Option<js_sys::ArrayBuffer>>,
}

impl xwt_core::session::stream::SendSpec for Session {
    type SendStream = SendStream;
}

impl xwt_core::session::stream::RecvSpec for Session {
    type RecvStream = RecvStream;
}

/// Send the data into a WebTransport stream.
pub struct SendStream {
    /// The WebTransport instance.
    pub transport: Rc<web_wt_sys::WebTransport>,
    /// The handle to the stream to write to.
    pub stream: web_wt_sys::WebTransportSendStream,
    /// A writer to conduct the operation.
    pub writer: web_sys_async_io::Writer,
}

/// Recv the data from a WebTransport stream.
pub struct RecvStream {
    /// The WebTransport instance.
    pub transport: Rc<web_wt_sys::WebTransport>,
    /// The handle to the stream to read from.
    pub stream: web_wt_sys::WebTransportReceiveStream,
    /// A reader to conduct the operation.
    pub reader: web_sys_async_io::Reader,
}

/// Open a reader for the given stream and create a [`RecvStream`].
fn wrap_recv_stream(
    transport: &Rc<web_wt_sys::WebTransport>,
    stream: web_wt_sys::WebTransportReceiveStream,
) -> RecvStream {
    let reader = web_sys_stream_utils::get_reader_byob(stream.clone());
    let reader: wasm_bindgen::JsValue = reader.into();
    let reader = reader.into();
    let reader = web_sys_async_io::Reader::new(reader);

    RecvStream {
        transport: Rc::clone(transport),
        stream,
        reader,
    }
}

/// Open a writer for the given stream and create a [`SendStream`].
fn wrap_send_stream(
    transport: &Rc<web_wt_sys::WebTransport>,
    stream: web_wt_sys::WebTransportSendStream,
) -> SendStream {
    let writer = stream.get_writer().unwrap();
    let writer = web_sys_async_io::Writer::new(writer.into());
    SendStream {
        transport: Rc::clone(transport),
        stream,
        writer,
    }
}

/// Take a bidi stream and wrap a reader and writer for it.
fn wrap_bi_stream(
    transport: &Rc<web_wt_sys::WebTransport>,
    stream: web_wt_sys::WebTransportBidirectionalStream,
) -> (SendStream, RecvStream) {
    let writeable = stream.writable();
    let readable = stream.readable();

    let send_stream = wrap_send_stream(transport, writeable);
    let recv_stream = wrap_recv_stream(transport, readable);

    (send_stream, recv_stream)
}

impl xwt_core::session::stream::OpenBi for Session {
    type Opening = xwt_core::utils::dummy::OpeningBiStream<Session>;

    type Error = Error;

    async fn open_bi(&self) -> Result<Self::Opening, Self::Error> {
        let value = self.transport.create_bidirectional_stream().await?;
        let value = wrap_bi_stream(&self.transport, value);
        Ok(xwt_core::utils::dummy::OpeningBiStream(value))
    }
}

impl xwt_core::session::stream::AcceptBi for Session {
    type Error = Error;

    async fn accept_bi(&self) -> Result<(Self::SendStream, Self::RecvStream), Self::Error> {
        let incoming: web_sys::ReadableStream = self.transport.incoming_bidirectional_streams();
        let reader: wasm_bindgen::JsValue = incoming.get_reader().into();
        let reader: web_sys::ReadableStreamDefaultReader = reader.into();
        let read_result = wasm_bindgen_futures::JsFuture::from(reader.read()).await?;
        let read_result: web_wt_sys::ReadableStreamReadResult = read_result.into();
        if read_result.is_done() {
            return Err(Error(JsError::new("xwt: accept bi reader is done").into()));
        }
        let value: web_wt_sys::WebTransportBidirectionalStream = read_result.get_value().into();
        let value = wrap_bi_stream(&self.transport, value);
        Ok(value)
    }
}

impl xwt_core::session::stream::OpenUni for Session {
    type Opening = xwt_core::utils::dummy::OpeningUniStream<Session>;
    type Error = Error;

    async fn open_uni(&self) -> Result<Self::Opening, Self::Error> {
        let value = self.transport.create_unidirectional_stream().await?;
        let send_stream = wrap_send_stream(&self.transport, value);
        Ok(xwt_core::utils::dummy::OpeningUniStream(send_stream))
    }
}

impl xwt_core::session::stream::AcceptUni for Session {
    type Error = Error;

    async fn accept_uni(&self) -> Result<Self::RecvStream, Self::Error> {
        let incoming: web_sys::ReadableStream = self.transport.incoming_unidirectional_streams();
        let reader: wasm_bindgen::JsValue = incoming.get_reader().into();
        let reader: web_sys::ReadableStreamDefaultReader = reader.into();
        let read_result = wasm_bindgen_futures::JsFuture::from(reader.read()).await?;
        let read_result: web_wt_sys::ReadableStreamReadResult = read_result.into();
        if read_result.is_done() {
            return Err(Error(JsError::new("xwt: accept uni reader is done").into()));
        }
        let value: web_wt_sys::WebTransportReceiveStream = read_result.get_value().into();
        let recv_stream = wrap_recv_stream(&self.transport, value);
        Ok(recv_stream)
    }
}

impl tokio::io::AsyncWrite for SendStream {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        std::pin::Pin::new(&mut self.writer).poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        std::pin::Pin::new(&mut self.writer).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        std::pin::Pin::new(&mut self.writer).poll_shutdown(cx)
    }
}

impl tokio::io::AsyncRead for RecvStream {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::pin::Pin::new(&mut self.reader).poll_read(cx, buf)
    }
}

impl xwt_core::stream::Write for SendStream {
    type Error = Error;

    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        web_sys_stream_utils::write(&self.writer.inner, buf).await?;
        Ok(buf.len())
    }
}

impl xwt_core::stream::Read for RecvStream {
    type Error = Error;

    async fn read(&mut self, buf: &mut [u8]) -> Result<Option<usize>, Self::Error> {
        let requested_size = buf.len().try_into().unwrap();
        let internal_buf = self
            .reader
            .internal_buf
            .take()
            .filter(|internal_buf| {
                let actual_size = internal_buf.byte_length();
                debug_assert!(actual_size > 0);
                actual_size >= requested_size
            })
            .unwrap_or_else(|| js_sys::ArrayBuffer::new(requested_size));
        let internal_buf_view = js_sys::Uint8Array::new(&internal_buf);
        let maybe_internal_buf_view =
            web_sys_stream_utils::read_byob(&self.reader.inner, internal_buf_view).await?;
        let Some(internal_buf_view) = maybe_internal_buf_view else {
            return Ok(None);
        };

        // Unwrap is safe assuming the `usize` is `u32` in wasm.
        let len = internal_buf_view.byte_length().try_into().unwrap();

        internal_buf_view.copy_to(&mut buf[..len]);

        self.reader.internal_buf = Some(internal_buf_view.buffer());

        Ok(Some(len))
    }
}

impl Session {
    /// Receive the datagram and handle the buffer with the given function.
    ///
    /// Cloning the buffer in the `f` will result in the undefined behaviour,
    /// because it will create a second reference to an object that is intended
    /// to be under a `mut ref`.
    /// Although is would not teachnically be unsafe, it would violate
    /// the borrow checker rules.
    pub async fn receive_datagram_with<R>(
        &self,
        f: impl FnOnce(&mut js_sys::Uint8Array) -> R,
    ) -> Result<R, Error> {
        let mut buffer_guard = self.datagram_read_buffer.lock().await;

        let buffer = buffer_guard
            .take()
            .unwrap_or_else(|| js_sys::ArrayBuffer::new(self.datagram_read_buffer_size));
        let view = js_sys::Uint8Array::new(&buffer);

        let maybe_view =
            web_sys_stream_utils::read_byob(&self.datagram_readable_stream_reader, view).await?;
        let Some(mut view) = maybe_view else {
            return Err(wasm_bindgen::JsError::new("unexpected stream termination").into());
        };

        let result = f(&mut view);

        *buffer_guard = Some(view.buffer());
        Ok(result)
    }
}

impl xwt_core::session::datagram::MaxSize for Session {
    fn max_datagram_size(&self) -> Option<usize> {
        let max_datagram_size = self.transport.datagrams().max_datagram_size();
        Some(usize::try_from(max_datagram_size).unwrap()) // u32 should fit in a usize on WASM
    }
}

impl xwt_core::session::datagram::Receive for Session {
    type Datagram = Vec<u8>;
    type Error = Error;

    async fn receive_datagram(&self) -> Result<Self::Datagram, Self::Error> {
        self.receive_datagram_with(|buffer| buffer.to_vec()).await
    }
}

impl xwt_core::session::datagram::ReceiveInto for Session {
    type Error = Error;

    async fn receive_datagram_into(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.receive_datagram_with(|buffer| {
            let len = buffer.length() as usize;
            buffer.copy_to(&mut buf[..len]);
            len
        })
        .await
    }
}

impl xwt_core::session::datagram::Send for Session {
    type Error = Error;

    async fn send_datagram<D>(&self, payload: D) -> Result<(), Self::Error>
    where
        D: AsRef<[u8]>,
    {
        web_sys_stream_utils::write(&self.datagram_writable_stream_writer, payload.as_ref())
            .await?;
        Ok(())
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        self.transport.close();
    }
}
