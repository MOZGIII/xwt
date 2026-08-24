use std::{future::Future, pin::Pin, task::Poll};

use wasm_bindgen_futures::JsFuture;

#[derive(Debug, Default)]
pub enum Op {
    #[default]
    Idle,
    ReadPending(JsFuture),
    ConsumingReadBuffer {
        read_buffer: js_sys::Uint8Array,
        already_read: usize,
    },
}

/// The underlying stream reader.
///
/// A BYOB reader is used when the stream is a byte stream; a default reader
/// is the fallback for the streams that do not support BYOB reads.
#[derive(Debug)]
pub enum Mode {
    /// A BYOB reader; reads into the owned internal buffer.
    Byob {
        /// The reader itself.
        reader: web_sys::ReadableStreamByobReader,

        /// The owned buffer to pass to the reads.
        ///
        /// [`None`] when the buffer ownership is currently transferred to
        /// a pending read, or when the buffer has not been allocated yet.
        internal_buf: Option<js_sys::ArrayBuffer>,
    },

    /// A default reader; yields stream-allocated chunks.
    Default {
        /// The reader itself.
        reader: web_sys::ReadableStreamDefaultReader,
    },
}

impl Mode {
    /// Release the stream lock held by the reader.
    pub fn release_lock(&self) {
        match self {
            Self::Byob { reader, .. } => reader.release_lock(),
            Self::Default { reader } => reader.release_lock(),
        }
    }

    /// Cancel the stream with the given reason.
    pub fn cancel_with_reason(&self, reason: &wasm_bindgen::JsValue) -> js_sys::Promise {
        match self {
            Self::Byob { reader, .. } => reader.cancel_with_reason(reason),
            Self::Default { reader } => reader.cancel_with_reason(reason),
        }
    }

    /// A promise that settles when the stream closes or errors.
    pub fn closed(&self) -> js_sys::Promise {
        match self {
            Self::Byob { reader, .. } => reader.closed(),
            Self::Default { reader } => reader.closed(),
        }
    }

    /// Start a read from the stream, returning the future of a read result
    /// (to be interpreted via [`parse_read_result`]).
    ///
    /// In the BYOB mode, passes the owned internal buffer to the read
    /// (allocating a new one if needed), requesting at most `requested_size`
    /// bytes; in the default mode, `requested_size` has no effect, as
    /// the stream decides the chunk sizes on its own.
    fn start_read(&mut self, requested_size: u32) -> JsFuture {
        match self {
            Self::Byob {
                reader,
                internal_buf,
            } => {
                let internal_buf = internal_buf
                    .take()
                    .filter(|internal_buf| {
                        let actual_size = internal_buf.byte_length();
                        debug_assert!(actual_size > 0);
                        actual_size >= requested_size
                    })
                    .unwrap_or_else(|| js_sys::ArrayBuffer::new(requested_size));
                let internal_buf_view = js_sys::Uint8Array::new_with_byte_offset_and_length(
                    &internal_buf,
                    0,
                    requested_size,
                );
                // Despite this not being properly indicated at the type system,
                // the `read_with_array_buffer_view` fn is actually supposed to
                // be taking the buffer by value - as it takes the ownership of
                // the buffer and the old JS reference to it is no longer valid.
                JsFuture::from(reader.read_with_array_buffer_view(&internal_buf_view))
            }
            Self::Default { reader } => JsFuture::from(reader.read()),
        }
    }

    /// Take the ownership of the buffer view returned from a read.
    ///
    /// In the BYOB mode, the buffer returned from a read is actually the same
    /// buffer we passed when we started the read - despite it being an
    /// entirely new JS object; we assume the ownership of the buffer and keep
    /// it for the next read.
    /// In the default mode this is a no-op, as the chunk buffers are
    /// stream-allocated.
    fn reclaim_read_buffer(&mut self, read_buffer: &js_sys::Uint8Array) {
        if let Self::Byob { internal_buf, .. } = self {
            *internal_buf = Some(read_buffer.buffer());
        }
    }
}

/// Interpret the settled result of a read started via
/// [`Mode::start_read`], extracting the read chunk.
///
/// Returns [`None`] when the stream is closed and has no more data.
fn parse_read_result(
    mode: &Mode,
    result: Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue>,
) -> Result<Option<js_sys::Uint8Array>, ReadError> {
    let result = result.map_err(ReadError::Read)?;
    let result: crate::sys::ReadableStreamReaderValue = result.into();
    match result.value() {
        Some(read_buffer) => Ok(Some(read_buffer)),
        // In the BYOB mode, the clean end of stream is a zero-length
        // buffer, and no buffer at all is an error condition; in
        // the default mode, no chunk is the clean end of stream.
        None => match mode {
            Mode::Byob { .. } => Err(ReadError::ByobReadConsumedBuffer),
            Mode::Default { .. } => Ok(None),
        },
    }
}

/// Copy the data from the read chunk into `dest`, returning the number of
/// bytes copied.
///
/// Keeps the chunk remainder that did not fit into `dest` (if any) in `op`
/// for the next read, and reclaims the chunk buffer into `mode` once
/// the chunk is fully consumed.
fn consume_read_buffer(
    mode: &mut Mode,
    op: &mut Op,
    read_buffer: js_sys::Uint8Array,
    already_read: usize,
    dest: &mut [u8],
) -> usize {
    let read_buffer_size = read_buffer.byte_length() as usize;
    let remaining_size = read_buffer_size - already_read;
    let copy_size = remaining_size.min(dest.len());

    // One JS-to-wasm copy per read is the minimum possible: the stream
    // cannot fill the wasm linear memory directly, as a BYOB read
    // transfers (detaches) the buffer backing the view it is given, and
    // the `WebAssembly.Memory` buffer is not detachable per spec.
    if already_read == 0 && copy_size == read_buffer_size {
        // The whole chunk is copied - no need for a subarray view.
        read_buffer.copy_to(&mut dest[..copy_size]);
    } else {
        let source_view =
            read_buffer.subarray(already_read as u32, (already_read + copy_size) as u32);
        source_view.copy_to(&mut dest[..copy_size]);
    }

    if already_read + copy_size < read_buffer_size {
        // Keep the chunk remainder for the next read.
        *op = Op::ConsumingReadBuffer {
            read_buffer,
            already_read: already_read + copy_size,
        };
    } else {
        mode.reclaim_read_buffer(&read_buffer);
    }

    copy_size
}

impl From<web_sys::ReadableStreamByobReader> for Mode {
    fn from(reader: web_sys::ReadableStreamByobReader) -> Self {
        Self::Byob {
            reader,
            internal_buf: None,
        }
    }
}

impl From<web_sys::ReadableStreamDefaultReader> for Mode {
    fn from(reader: web_sys::ReadableStreamDefaultReader) -> Self {
        Self::Default { reader }
    }
}

/// An error that can occur when reading via [`Reader::read_into`].
#[derive(Debug)]
pub enum ReadError {
    /// The underlying read operation threw an error.
    Read(wasm_bindgen::JsValue),

    /// A BYOB read consumed the buffer and did not provide a new one;
    /// this indicates an error condition.
    ByobReadConsumedBuffer,
}

impl From<ReadError> for std::io::Error {
    fn from(err: ReadError) -> Self {
        match err {
            ReadError::Read(err) => super::js_value_to_io_error(err),
            ReadError::ByobReadConsumedBuffer => {
                std::io::Error::other("BYOB read consumed the buffer and did not provide a new one")
            }
        }
    }
}

#[derive(Debug)]
pub struct Reader {
    pub inner: Mode,
    pub op: Op,
}

impl Reader {
    pub fn new(inner: impl Into<Mode>) -> Self {
        Self {
            inner: inner.into(),
            op: Op::default(),
        }
    }

    pub fn with_buf(
        inner: web_sys::ReadableStreamByobReader,
        internal_buf: js_sys::ArrayBuffer,
    ) -> Self {
        Self {
            inner: Mode::Byob {
                reader: inner,
                internal_buf: Some(internal_buf),
            },
            op: Op::default(),
        }
    }

    /// Read from the stream into the given buffer, returning the number of
    /// bytes read.
    ///
    /// Returns `Ok(0)` when the stream is closed and has no more data.
    ///
    /// Consumes the leftovers of the chunks that did not fit into the buffers
    /// of the previous reads, and resolves the reads left pending by
    /// a dropped [`tokio::io::AsyncRead::poll_read`].
    pub async fn read_into(&mut self, buf: &mut [u8]) -> Result<usize, ReadError> {
        // Take the leftover of a previously read chunk, or read a new chunk
        // from the stream.
        let (read_buffer, already_read) = match std::mem::take(&mut self.op) {
            Op::ConsumingReadBuffer {
                read_buffer,
                already_read,
            } => (read_buffer, already_read),
            op => {
                let fut = match op {
                    // Resolve the read left pending by a dropped `poll_read`.
                    Op::ReadPending(fut) => fut,
                    // Start a new read.
                    _ => {
                        let requested_size = buf.len().try_into().unwrap();
                        self.inner.start_read(requested_size)
                    }
                };
                match parse_read_result(&self.inner, fut.await)? {
                    Some(read_buffer) => (read_buffer, 0),
                    None => return Ok(0),
                }
            }
        };

        Ok(consume_read_buffer(
            &mut self.inner,
            &mut self.op,
            read_buffer,
            already_read,
            buf,
        ))
    }
}

impl tokio::io::AsyncRead for Reader {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();

        // A read into a buffer with no remaining capacity must complete
        // immediately without requesting more data from the stream.
        if buf.remaining() == 0 {
            return Poll::Ready(Ok(()));
        }

        match std::mem::take(&mut this.op) {
            Op::ReadPending(mut fut) => {
                let result = match Pin::new(&mut fut).poll(cx) {
                    Poll::Pending => {
                        this.op = Op::ReadPending(fut);
                        return Poll::Pending;
                    }
                    Poll::Ready(result) => result,
                };

                // No chunk indicates the end of stream.
                let Some(read_buffer) = parse_read_result(&this.inner, result)? else {
                    return Poll::Ready(Ok(()));
                };

                this.op = Op::ConsumingReadBuffer {
                    read_buffer,
                    already_read: 0,
                };

                Pin::new(this).poll_read(cx, buf)
            }
            Op::ConsumingReadBuffer {
                read_buffer,
                already_read,
            } => {
                let remaining_size = read_buffer.byte_length() as usize - already_read;
                let copy_size = remaining_size.min(buf.remaining());

                let write_slice = buf.initialize_unfilled_to(copy_size);
                let copied = consume_read_buffer(
                    &mut this.inner,
                    &mut this.op,
                    read_buffer,
                    already_read,
                    write_slice,
                );
                buf.advance(copied);

                Poll::Ready(Ok(()))
            }
            Op::Idle => {
                let requested_size = buf.remaining().try_into().unwrap();
                let fut = this.inner.start_read(requested_size);
                this.op = Op::ReadPending(fut);
                Pin::new(this).poll_read(cx, buf)
            }
        }
    }
}
