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
    pub internal_buf: Option<js_sys::Uint8Array>,
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
        internal_buf: js_sys::Uint8Array,
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
            Op::Pending(ref mut fut) => {
                let result = ready!(Pin::new(fut).poll(cx));
                self.op = Op::Idle;

                let read_result = match result {
                    Ok(val) => val,
                    Err(err) => return Poll::Ready(Err(super::js_value_to_io_error(err))),
                };
                let read_result: crate::sys::ReadableStreamByobReaderValue = read_result.into();

                let value = read_result.value();
                // No value indicates an error condition.
                let Some(internal_buf) = value else {
                    return Poll::Ready(Ok(()));
                };

                let len = wasm_u32_to_usize(internal_buf.byte_length());

                let write_slice = buf.initialize_unfilled_to(len);
                internal_buf.copy_to(&mut write_slice[..len]);
                buf.advance(len);

                self.internal_buf = Some(internal_buf);

                Poll::Ready(Ok(()))
            }
            Op::Idle => {
                let internal_buf = self.internal_buf.get_or_insert_with(|| {
                    js_sys::Uint8Array::new_with_length(buf.capacity().try_into().unwrap())
                });
                let internal_buf = internal_buf.clone(); // this only clones the reference
                let fut = JsFuture::from(self.inner.read_with_array_buffer_view(&internal_buf));
                self.op = Op::Pending(fut);
                self.poll_read(cx, buf)
            }
        }
    }
}

#[inline]
fn wasm_u32_to_usize(val: u32) -> usize {
    val.try_into().unwrap()
}
