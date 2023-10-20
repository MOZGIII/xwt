use std::{
    future::Future,
    pin::Pin,
    task::{ready, Poll},
};

use wasm_bindgen_futures::JsFuture;

use crate::Op;

#[derive(Debug)]
pub struct Reader {
    pub inner: web_sys::ReadableStreamDefaultReader,
    pub op: Op,
}

impl Reader {
    pub fn new(inner: web_sys::ReadableStreamDefaultReader) -> Self {
        Self {
            inner,
            op: Op::default(),
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
                let read_result: crate::sys::ReadableStreamDefaultReaderValue = read_result.into();

                let value = read_result.value();
                // No value indicates an error condition.
                let Some(js_buf) = value else {
                    return Poll::Ready(Ok(()));
                };

                let data = js_buf.to_vec();
                buf.put_slice(&data);

                Poll::Ready(Ok(()))
            }
            Op::Idle => {
                let fut = JsFuture::from(self.inner.read());
                self.op = Op::Pending(fut);
                self.poll_read(cx, buf)
            }
        }
    }
}
