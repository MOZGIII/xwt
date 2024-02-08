#![cfg_attr(
    target_family = "wasm",
    doc = "The [`web_sys`]-powered implementation of [`xwt_core`]."
)]
#![cfg_attr(
    not(target_family = "wasm"),
    doc = "The `web_sys`-powered implementation of `xwt_core`."
)]
#![cfg(target_family = "wasm")]

use std::rc::Rc;

use wasm_bindgen::JsError;
use xwt_core::async_trait;

mod error;
mod sys;

pub use error::*;

/// An endpoint for the xwt.
///
/// Internally holds the connection options and can create
/// a new `WebTransport` object on the "web" side on a connection request.
#[derive(Debug, Clone, Default)]
pub struct Endpoint {
    /// The options to use to create the `WebTransport`s.
    pub options: web_sys::WebTransportOptions,
}

#[async_trait(?Send)]
impl xwt_core::traits::EndpointConnect for Endpoint {
    type Error = Error;
    type Connecting = xwt_core::utils::dummy::Connecting<Connection>;

    async fn connect(&self, url: &str) -> Result<Self::Connecting, Self::Error> {
        let transport = web_sys::WebTransport::new_with_options(url, &self.options)?;
        let _ = wasm_bindgen_futures::JsFuture::from(transport.ready()).await?;

        let datagrams = transport.datagrams();
        let max_datagram_size = datagrams.max_datagram_size();
        let datagram_readable_stream_reader =
            web_sys_stream_utils::get_reader_byob(datagrams.readable());
        let datagram_writable_stream_writer =
            web_sys_stream_utils::get_writer(datagrams.writable());

        let datagram_read_buffer = js_sys::Uint8Array::new_with_length(max_datagram_size);
        let datagram_read_buffer = tokio::sync::Mutex::new(Some(datagram_read_buffer));

        let connection = Connection {
            transport: Rc::new(transport),
            datagram_readable_stream_reader,
            datagram_writable_stream_writer,
            max_datagram_size,
            datagram_read_buffer,
        };
        Ok(xwt_core::utils::dummy::Connecting(connection))
    }
}

/// Connection holds the [`web_sys::WebTransport`] and is responsible for
/// providing access to the Web API of WebTransport in a way that is portable.
/// It also holds handles to the datagram reader and writer, as well as
/// the datagram reader state.
#[derive(Debug)]
pub struct Connection {
    /// The WebTransport instance.
    pub transport: Rc<web_sys::WebTransport>,
    /// The datagram reader.
    pub datagram_readable_stream_reader: web_sys::ReadableStreamByobReader,
    /// The datagram writer.
    pub datagram_writable_stream_writer: web_sys::WritableStreamDefaultWriter,
    /// The max datagram size.
    /// Used to allocate the datagram read buffer in case it gets lost.
    pub max_datagram_size: u32,
    /// The datagram read internal buffer.
    pub datagram_read_buffer: tokio::sync::Mutex<Option<js_sys::Uint8Array>>,
}

impl xwt_core::traits::Streams for Connection {
    type SendStream = SendStream;
    type RecvStream = RecvStream;
}

/// Send the data into a WebTransport stream.
pub struct SendStream {
    /// The WebTransport instance.
    pub transport: Rc<web_sys::WebTransport>,
    /// The handle to the stream to write to.
    pub stream: web_sys::WebTransportSendStream,
    /// A writer to conduct the operation.
    pub writer: web_sys_async_io::Writer,
}

/// Recv the data from a WebTransport stream.
pub struct RecvStream {
    /// The WebTransport instance.
    pub transport: Rc<web_sys::WebTransport>,
    /// The handle to the stream to read from.
    pub stream: web_sys::WebTransportReceiveStream,
    /// A reader to conduct the operation.
    pub reader: web_sys_async_io::Reader,
}

/// Open a reader for the given stream and create a [`RecvStream`].
fn wrap_recv_stream(
    transport: &Rc<web_sys::WebTransport>,
    stream: web_sys::WebTransportReceiveStream,
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
    transport: &Rc<web_sys::WebTransport>,
    stream: web_sys::WebTransportSendStream,
) -> SendStream {
    let writer = stream.get_writer().unwrap();
    let writer = web_sys_async_io::Writer::new(writer);
    SendStream {
        transport: Rc::clone(transport),
        stream,
        writer,
    }
}

/// Take a bidi stream and wrap a reader and writer for it.
fn wrap_bi_stream(
    transport: &Rc<web_sys::WebTransport>,
    stream: web_sys::WebTransportBidirectionalStream,
) -> (SendStream, RecvStream) {
    let writeable = stream.writable();
    let readable = stream.readable();

    let send_stream = wrap_send_stream(transport, writeable);
    let recv_stream = wrap_recv_stream(transport, readable);

    (send_stream, recv_stream)
}

#[async_trait(?Send)]
impl xwt_core::traits::OpenBiStream for Connection {
    type Opening = xwt_core::utils::dummy::OpeningBiStream<Connection>;

    type Error = Error;

    async fn open_bi(&self) -> Result<Self::Opening, Self::Error> {
        let value =
            wasm_bindgen_futures::JsFuture::from(self.transport.create_bidirectional_stream())
                .await?;
        let value: web_sys::WebTransportBidirectionalStream = value.into();
        let value = wrap_bi_stream(&self.transport, value);
        Ok(xwt_core::utils::dummy::OpeningBiStream(value))
    }
}

#[async_trait(?Send)]
impl xwt_core::traits::AcceptBiStream for Connection {
    type Error = Error;

    async fn accept_bi(&self) -> Result<(Self::SendStream, Self::RecvStream), Self::Error> {
        let incoming: web_sys::ReadableStream = self.transport.incoming_bidirectional_streams();
        let reader: wasm_bindgen::JsValue = incoming.get_reader().into();
        let reader: web_sys::ReadableStreamDefaultReader = reader.into();
        let read_result = wasm_bindgen_futures::JsFuture::from(reader.read()).await?;
        let read_result: crate::sys::ReadableStreamReadResult = read_result.into();
        if read_result.is_done() {
            return Err(Error(JsError::new("xwt: accept bi reader is done").into()));
        }
        let value: web_sys::WebTransportBidirectionalStream = read_result.value().into();
        let value = wrap_bi_stream(&self.transport, value);
        Ok(value)
    }
}

#[async_trait(?Send)]
impl xwt_core::traits::OpenUniStream for Connection {
    type Opening = xwt_core::utils::dummy::OpeningUniStream<Connection>;
    type Error = Error;

    async fn open_uni(&self) -> Result<Self::Opening, Self::Error> {
        let value =
            wasm_bindgen_futures::JsFuture::from(self.transport.create_unidirectional_stream())
                .await?;
        let value: web_sys::WebTransportSendStream = value.into();
        let send_stream = wrap_send_stream(&self.transport, value);
        Ok(xwt_core::utils::dummy::OpeningUniStream(send_stream))
    }
}

#[async_trait(?Send)]
impl xwt_core::traits::AcceptUniStream for Connection {
    type Error = Error;

    async fn accept_uni(&self) -> Result<Self::RecvStream, Self::Error> {
        let incoming: web_sys::ReadableStream = self.transport.incoming_unidirectional_streams();
        let reader: wasm_bindgen::JsValue = incoming.get_reader().into();
        let reader: web_sys::ReadableStreamDefaultReader = reader.into();
        let read_result = wasm_bindgen_futures::JsFuture::from(reader.read()).await?;
        let read_result: crate::sys::ReadableStreamReadResult = read_result.into();
        if read_result.is_done() {
            return Err(Error(JsError::new("xwt: accept uni reader is done").into()));
        }
        let value: web_sys::WebTransportReceiveStream = read_result.value().into();
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

#[async_trait(?Send)]
impl xwt_core::io::Write for SendStream {
    type Error = Error;

    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        web_sys_stream_utils::write(&self.writer.inner, buf).await?;
        Ok(buf.len())
    }
}

#[async_trait(?Send)]
impl xwt_core::io::Read for RecvStream {
    type Error = Error;

    async fn read(&mut self, buf: &mut [u8]) -> Result<Option<usize>, Self::Error> {
        let internal_buf =
            self.reader.internal_buf.take().unwrap_or_else(|| {
                js_sys::Uint8Array::new_with_length(buf.len().try_into().unwrap())
            });
        let maybe_internal_buf =
            web_sys_stream_utils::read_byob(&self.reader.inner, internal_buf).await?;
        let Some(internal_buf) = maybe_internal_buf else {
            return Ok(None);
        };

        // Unwrap is safe assuming the `usize` is `u32` in wasm.
        let len = internal_buf.byte_length().try_into().unwrap();

        internal_buf.copy_to(&mut buf[..len]);

        self.reader.internal_buf = Some(internal_buf);

        Ok(Some(len))
    }
}

#[async_trait(?Send)]
impl xwt_core::datagram::Receive for Connection {
    type Datagram = Vec<u8>;
    type Error = Error;

    async fn receive_datagram(&self) -> Result<Self::Datagram, Self::Error> {
        let mut buffer_guard = self.datagram_read_buffer.lock().await;

        let buffer = buffer_guard
            .take()
            .unwrap_or_else(|| js_sys::Uint8Array::new_with_length(self.max_datagram_size));

        let maybe_buffer =
            web_sys_stream_utils::read_byob(&self.datagram_readable_stream_reader, buffer).await?;
        let Some(buffer) = maybe_buffer else {
            return Err(wasm_bindgen::JsError::new("unexpected stream termination").into());
        };

        let data = buffer.to_vec();
        *buffer_guard = Some(buffer);

        Ok(data)
    }
}

#[async_trait(?Send)]
impl xwt_core::datagram::Send for Connection {
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

impl Drop for Connection {
    fn drop(&mut self) {
        self.transport.close();
    }
}
