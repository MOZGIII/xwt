#![cfg_attr(
    target_family = "wasm",
    doc = "The [`web_wt_sys`]-powered implementation of [`xwt_core`]."
)]
#![cfg_attr(
    not(target_family = "wasm"),
    doc = "The `web_wt_sys`-powered implementation of `xwt_core`."
)]
#![cfg(target_family = "wasm")]

use std::{num::NonZeroUsize, rc::Rc};

use wasm_bindgen::prelude::*;

mod error;
mod error_as_error_code;
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

        Ok(Session::new(transport))
    }
}

/// Session holds the [`web_wt_sys::WebTransport`] and is responsible for
/// providing access to the Web API of WebTransport in a way that is portable.
/// It also holds handles to the datagram reader and writer, as well as
/// the datagram reader state.
#[derive(Debug)]
pub struct Session {
    /// The WebTransport instance.
    transport: Option<Rc<web_wt_sys::WebTransport>>,

    /// The datagrams state for this session.
    pub datagrams: Datagrams,

    /// Whether to close the session on drop.
    pub close_on_drop: bool,
}

impl Session {
    /// Construct a new session from a [`web_wt_sys::WebTransport`].
    pub fn new(transport: web_wt_sys::WebTransport) -> Self {
        let datagrams = Datagrams::from_transport(&transport);
        Self {
            transport: Some(Rc::new(transport)),
            datagrams,
            close_on_drop: true,
        }
    }

    /// If possible, relieves the underlying [`web_wt_sys::WebTransport`] of
    /// any [`xwt_web`]-held locks and dependencies and exposes it.
    pub fn try_unwrap(mut self) -> Result<web_wt_sys::WebTransport, Self> {
        // Take the transport out of the session; this state is not valid
        // "publicly", only while we're in this fn.
        let transport = self.transport.take().unwrap();

        // We want to ensure there are no other references to
        // the transport's [`Rc`], otherwise something must still be using it.
        // If we can't unwrap it successfully - we reinsert the transport back
        // into `self` and return it as an `Err` - it will thus remain
        // operational to permit doing whatever work may have to be done
        // further.
        let unwrapped = match Rc::try_unwrap(transport) {
            Ok(unwrapped) => unwrapped,
            Err(transport) => {
                let _ = self.transport.insert(transport);
                return Err(self);
            }
        };

        // Do not close the transport (we have taken it out anyway).
        self.close_on_drop = false;

        // Drop the session to release the datagram readers/writers.
        drop(self);

        // Return the unwrapped transport.
        Ok(unwrapped)
    }

    /// Obtain a transport ref.
    pub const fn transport_ref(&self) -> &Rc<web_wt_sys::WebTransport> {
        // Trnasport should never be gone generally, only inside of
        // the `try_unwrap`.
        self.transport.as_ref().unwrap()
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        if self.close_on_drop {
            self.transport_ref().close();
        }
    }
}

/// Datagrams hold the portions of the session that are responsible for working
/// with the datagrams.
#[derive(Debug)]
pub struct Datagrams {
    /// The datagram reader.
    pub readable_stream_reader: web_sys::ReadableStreamByobReader,

    /// The datagram writer.
    pub writable_stream_writer: web_sys::WritableStreamDefaultWriter,

    /// The desired size of the datagram read buffer.
    /// Used to allocate the datagram read buffer in case it gets lost.
    pub read_buffer_size: u32,

    /// The datagram read internal buffer.
    pub read_buffer: tokio::sync::Mutex<Option<js_sys::ArrayBuffer>>,

    /// Unlock the streams on drop.
    pub unlock_streams_on_drop: bool,
}

impl Datagrams {
    /// Create a datagrams state from the transport.
    pub fn from_transport(transport: &web_wt_sys::WebTransport) -> Self {
        Self::from_transport_datagrams(&transport.datagrams())
    }

    /// Create a datagrams state from the transport datagrams.
    pub fn from_transport_datagrams(
        datagrams: &web_wt_sys::WebTransportDatagramDuplexStream,
    ) -> Self {
        let read_buffer_size = 65536; // 65k buffers as per spec recommendation

        let readable_stream_reader = web_sys_stream_utils::get_reader_byob(datagrams.readable());
        let writable_stream_writer = web_sys_stream_utils::get_writer(datagrams.writable());

        let read_buffer = js_sys::ArrayBuffer::new(read_buffer_size);
        let read_buffer = tokio::sync::Mutex::new(Some(read_buffer));

        Self {
            readable_stream_reader,
            writable_stream_writer,
            read_buffer_size,
            read_buffer,
            unlock_streams_on_drop: true,
        }
    }
}

impl Drop for Datagrams {
    fn drop(&mut self) {
        if self.unlock_streams_on_drop {
            self.readable_stream_reader.release_lock();
            self.writable_stream_writer.release_lock();
        }
    }
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

impl Drop for SendStream {
    fn drop(&mut self) {
        self.writer.inner.release_lock();
    }
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

impl Drop for RecvStream {
    fn drop(&mut self) {
        self.reader.inner.release_lock();
    }
}

/// Open a reader for the given stream and create a [`RecvStream`].
fn wrap_recv_stream(
    transport: &Rc<web_wt_sys::WebTransport>,
    stream: web_wt_sys::WebTransportReceiveStream,
) -> RecvStream {
    let reader = web_sys_stream_utils::get_reader_byob(stream.clone());
    let reader: JsValue = reader.into();
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
        let transport = self.transport_ref();
        let value = transport.create_bidirectional_stream().await?;
        let value = wrap_bi_stream(transport, value);
        Ok(xwt_core::utils::dummy::OpeningBiStream(value))
    }
}

impl xwt_core::session::stream::AcceptBi for Session {
    type Error = Error;

    async fn accept_bi(&self) -> Result<(Self::SendStream, Self::RecvStream), Self::Error> {
        let transport = self.transport_ref();
        let incoming: web_sys::ReadableStream = transport.incoming_bidirectional_streams();
        let reader: JsValue = incoming.get_reader().into();
        let reader: web_sys::ReadableStreamDefaultReader = reader.into();
        let read_result = wasm_bindgen_futures::JsFuture::from(reader.read()).await?;
        let read_result: web_wt_sys::ReadableStreamReadResult = read_result.into();
        if read_result.is_done() {
            return Err(Error(JsError::new("xwt: accept bi reader is done").into()));
        }
        let value: web_wt_sys::WebTransportBidirectionalStream = read_result.get_value().into();
        let value = wrap_bi_stream(transport, value);
        Ok(value)
    }
}

impl xwt_core::session::stream::OpenUni for Session {
    type Opening = xwt_core::utils::dummy::OpeningUniStream<Session>;
    type Error = Error;

    async fn open_uni(&self) -> Result<Self::Opening, Self::Error> {
        let transport = self.transport_ref();
        let value = transport.create_unidirectional_stream().await?;
        let send_stream = wrap_send_stream(transport, value);
        Ok(xwt_core::utils::dummy::OpeningUniStream(send_stream))
    }
}

impl xwt_core::session::stream::AcceptUni for Session {
    type Error = Error;

    async fn accept_uni(&self) -> Result<Self::RecvStream, Self::Error> {
        let transport = self.transport_ref();
        let incoming: web_sys::ReadableStream = transport.incoming_unidirectional_streams();
        let reader: JsValue = incoming.get_reader().into();
        let reader: web_sys::ReadableStreamDefaultReader = reader.into();
        let read_result = wasm_bindgen_futures::JsFuture::from(reader.read()).await?;
        let read_result: web_wt_sys::ReadableStreamReadResult = read_result.into();
        if read_result.is_done() {
            return Err(Error(JsError::new("xwt: accept uni reader is done").into()));
        }
        let value: web_wt_sys::WebTransportReceiveStream = read_result.get_value().into();
        let recv_stream = wrap_recv_stream(transport, value);
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

/// An error that can occur during the stream writes.
#[derive(Debug, thiserror::Error)]
pub enum StreamWriteError {
    /// The write was called with a zero-size write buffer.
    #[error("zero size write buffer")]
    ZeroSizeWriteBuffer,

    /// The write call thrown an exception.
    #[error("write error: {0}")]
    Write(Error),
}

impl xwt_core::stream::Write for SendStream {
    type Error = StreamWriteError;

    async fn write(&mut self, buf: &[u8]) -> Result<NonZeroUsize, Self::Error> {
        let Some(buf_len) = NonZeroUsize::new(buf.len()) else {
            return Err(StreamWriteError::ZeroSizeWriteBuffer);
        };

        web_sys_stream_utils::write(&self.writer.inner, buf)
            .await
            .map_err(|err| StreamWriteError::Write(err.into()))?;

        Ok(buf_len)
    }
}

impl xwt_core::stream::WriteAbort for SendStream {
    type Error = Error;

    async fn abort(self, error_code: xwt_core::stream::ErrorCode) -> Result<(), Self::Error> {
        wasm_bindgen_futures::JsFuture::from(
            self.writer.inner.abort_with_reason(&error_code.into()),
        )
        .await
        .map(|val| {
            debug_assert!(val.is_undefined());
        })
        .map_err(Error::from)
    }
}

impl xwt_core::stream::WriteAborted for SendStream {
    type Error = Error;

    async fn aborted(self) -> Result<xwt_core::stream::ErrorCode, Self::Error> {
        // Hack our way through...
        let result = wasm_bindgen_futures::JsFuture::from(self.writer.inner.closed()).await;
        match result {
            Ok(value) => {
                debug_assert!(value.is_undefined());
                Ok(0)
            }
            Err(value) => {
                let error: web_wt_sys::WebTransportError = value.dyn_into().unwrap();
                if error.source() != web_wt_sys::WebTransportErrorSource::Stream {
                    return Err(Error(error.into()));
                }
                let Some(code) = error.stream_error_code() else {
                    return Err(Error(error.into()));
                };
                Ok(code)
            }
        }
    }
}

impl xwt_core::stream::Finish for SendStream {
    type Error = Error;

    async fn finish(self) -> Result<(), Self::Error> {
        wasm_bindgen_futures::JsFuture::from(self.writer.inner.close())
            .await
            .map(|val| {
                debug_assert!(val.is_undefined());
            })
            .map_err(Error::from)
    }
}

impl xwt_core::stream::Finished for RecvStream {
    type Error = Error;

    async fn finished(self) -> Result<(), Self::Error> {
        wasm_bindgen_futures::JsFuture::from(self.reader.inner.closed())
            .await
            .map(|val| {
                debug_assert!(val.is_undefined());
            })
            .map_err(Error::from)
    }
}

/// An error that can occur while reading stream data.
#[derive(Debug, thiserror::Error)]
pub enum StreamReadError {
    /// This is an odd case, which is still tbd what to do with.
    #[error("byob read consumed the buffer and didn't provide a new one")]
    ByobReadConsumedBuffer,

    /// The `read_byob` call thrown an exception.
    #[error("read error: {0}")]
    Read(Error),

    /// The stream was closed, and there is no more data to exect there.
    #[error("stream closed")]
    Closed,
}

impl xwt_core::stream::Read for RecvStream {
    type Error = StreamReadError;

    async fn read(&mut self, buf: &mut [u8]) -> Result<NonZeroUsize, Self::Error> {
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
        let internal_buf_view =
            js_sys::Uint8Array::new_with_byte_offset_and_length(&internal_buf, 0, requested_size);
        let maybe_internal_buf_view =
            web_sys_stream_utils::read_byob(&self.reader.inner, internal_buf_view)
                .await
                .map_err(|err| StreamReadError::Read(err.into()))?;
        let Some(internal_buf_view) = maybe_internal_buf_view else {
            return Err(StreamReadError::ByobReadConsumedBuffer);
        };

        // Unwrap is safe assuming the `usize` is `u32` in wasm.
        let len = internal_buf_view.byte_length().try_into().unwrap();

        // Detect when the read is aborted because the stream was closed without
        // an error.
        let Some(len) = NonZeroUsize::new(len) else {
            return Err(StreamReadError::Closed);
        };

        internal_buf_view.copy_to(&mut buf[..len.get()]);

        self.reader.internal_buf = Some(internal_buf_view.buffer());

        Ok(len)
    }
}

impl xwt_core::stream::ReadAbort for RecvStream {
    type Error = Error;

    async fn abort(self, error_code: xwt_core::stream::ErrorCode) -> Result<(), Self::Error> {
        wasm_bindgen_futures::JsFuture::from(
            self.reader.inner.cancel_with_reason(&error_code.into()),
        )
        .await
        .map(|_| ())
        .map_err(Error::from)
    }
}

impl xwt_core::stream::ReadAborted for RecvStream {
    type Error = Error;

    async fn aborted(self) -> Result<xwt_core::stream::ErrorCode, Self::Error> {
        // Hack our way through...
        let result = wasm_bindgen_futures::JsFuture::from(self.reader.inner.closed()).await;
        match result {
            Ok(value) => {
                debug_assert!(value.is_undefined());
                Ok(0)
            }
            Err(value) => {
                let error: web_wt_sys::WebTransportError = value.dyn_into().unwrap();
                if error.source() != web_wt_sys::WebTransportErrorSource::Stream {
                    return Err(Error(error.into()));
                }
                let Some(code) = error.stream_error_code() else {
                    return Err(Error(error.into()));
                };
                Ok(code)
            }
        }
    }
}

impl Datagrams {
    /// Receive the datagram and handle the buffer with the given function.
    ///
    /// Cloning the buffer in the `f` will result in the undefined behaviour,
    /// because it will create a second reference to an object that is intended
    /// to be under a `mut ref`.
    /// Although is would not teachnically be unsafe, it would violate
    /// the borrow checker rules.
    pub async fn receive_with<R>(
        &self,
        max_read_size: Option<u32>,
        f: impl FnOnce(&mut js_sys::Uint8Array) -> R,
    ) -> Result<R, Error> {
        let mut buffer_guard = self.read_buffer.lock().await;

        let buffer = buffer_guard
            .take()
            .unwrap_or_else(|| js_sys::ArrayBuffer::new(self.read_buffer_size));
        let view = if let Some(max_read_size) = max_read_size {
            let desired_buffer_length = buffer.byte_length().min(max_read_size);
            js_sys::Uint8Array::new_with_byte_offset_and_length(&buffer, 0, desired_buffer_length)
        } else {
            js_sys::Uint8Array::new(&buffer)
        };

        let maybe_view =
            web_sys_stream_utils::read_byob(&self.readable_stream_reader, view).await?;
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
        let transport = self.transport_ref();
        let max_datagram_size = transport.datagrams().max_datagram_size();
        Some(usize::try_from(max_datagram_size).unwrap()) // u32 should fit in a usize on WASM
    }
}

impl xwt_core::session::datagram::Receive for Session {
    type Datagram = Vec<u8>;
    type Error = Error;

    async fn receive_datagram(&self) -> Result<Self::Datagram, Self::Error> {
        self.datagrams
            .receive_with(None, |buffer| buffer.to_vec())
            .await
    }
}

impl xwt_core::session::datagram::ReceiveInto for Session {
    type Error = Error;

    async fn receive_datagram_into(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let max_read_size = buf.len().try_into().unwrap();
        self.datagrams
            .receive_with(Some(max_read_size), |buffer| {
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
        web_sys_stream_utils::write(&self.datagrams.writable_stream_writer, payload.as_ref())
            .await?;
        Ok(())
    }
}
