use std::{
    future::Future,
    pin::Pin,
    task::{ready, Poll},
};

use wasm_bindgen_futures::JsFuture;

use crate::Op;

#[derive(Debug)]
pub struct Reader {
    pub inner: web_sys::ReadableStreamByobReader,
    pub op: Op,
    pub internal_buf: Option<js_sys::ArrayBuffer>,
}

impl Reader {
    pub fn new(inner: web_sys::ReadableStreamByobReader) -> Self {
        Self {
            inner,
            op: Op::default(),
            internal_buf: None,
        }
    }

    pub fn with_buf(
        inner: web_sys::ReadableStreamByobReader,
        internal_buf: js_sys::ArrayBuffer,
    ) -> Self {
        Self {
            inner,
            op: Op::default(),
            internal_buf: Some(internal_buf),
        }
    }
}

impl tokio::io::AsyncRead for Reader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match self.op {
            Op::Read(ref mut fut) => {
                let result = ready!(Pin::new(fut).poll(cx));
                self.op = Op::Idle;

                let read_result = match result {
                    Ok(val) => val,
                    Err(err) => return Poll::Ready(Err(super::js_value_to_io_error(err))),
                };
                let read_result: crate::sys::ReadableStreamByobReaderValue = read_result.into();

                let value = read_result.value();
                // No value indicates an error condition.
                let Some(internal_buf_view) = value else {
                    return Poll::Ready(Ok(()));
                };

                let len = wasm_u32_to_usize(internal_buf_view.byte_length());

                let write_slice = buf.initialize_unfilled_to(len);
                internal_buf_view.copy_to(&mut write_slice[..len]);
                buf.advance(len);

                // The buffer returned is actually the same buffer we passed
                // earlier when we called the `read_with_array_buffer_view`
                // under the hood - despite it being an entirely new JS object.
                // We now have to assume the ownership of the buffer and
                // properly keep for the next time.
                self.internal_buf = Some(internal_buf_view.buffer());

                Poll::Ready(Ok(()))
            }
            Op::Idle => {
                let requested_size = buf.capacity().try_into().unwrap();
                let internal_buf = self
                    .internal_buf
                    .take()
                    .filter(|internal_buf| {
                        let actual_size = internal_buf.byte_length();
                        debug_assert!(actual_size > 0);
                        actual_size >= requested_size
                    })
                    .unwrap_or_else(|| js_sys::ArrayBuffer::new(requested_size));
                let internal_buf_view = js_sys::Uint8Array::new(&internal_buf);
                // Despite this not being properly indicated at the type system,
                // the `read_with_array_buffer_view` fn is actually supposed
                // to be taking the buffer by value - as it takes the ownership of
                // the buffer and the old JS reference to it is no longer valid.
                let fut =
                    JsFuture::from(self.inner.read_with_array_buffer_view(&internal_buf_view));
                self.op = Op::Read(fut);
                self.poll_read(cx, buf)
            }
            _ => {
                unreachable!("Reader should only have Read or Idle operations")
            }
        }
    }
}

#[inline]
fn wasm_u32_to_usize(val: u32) -> usize {
    val.try_into().unwrap()
}
