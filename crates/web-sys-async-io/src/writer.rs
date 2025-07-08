use std::{
    future::Future,
    pin::Pin,
    task::{ready, Poll},
};

use wasm_bindgen_futures::JsFuture;

use crate::WriterOp;

#[derive(Debug)]
pub struct Writer {
    pub inner: web_sys::WritableStreamDefaultWriter,
    pub op: WriterOp,
}

impl Writer {
    pub fn new(inner: web_sys::WritableStreamDefaultWriter) -> Self {
        Self {
            inner,
            op: WriterOp::default(),
        }
    }
}

impl tokio::io::AsyncWrite for Writer {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        match self.op {
            WriterOp::Write(ref mut fut, size) => {
                let result = ready!(Pin::new(fut).poll(cx));
                self.op = WriterOp::Idle;
                Poll::Ready(result.map(|_| size).map_err(super::js_value_to_io_error))
            }
            WriterOp::Idle => {
                let chunk = js_sys::Uint8Array::from(buf);
                let fut = JsFuture::from(self.inner.write_with_chunk(chunk.as_ref()));
                self.op = WriterOp::Write(fut, buf.len());
                self.poll_write(cx, buf)
            }
            _ => Poll::Pending,
        }
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        match self.op {
            WriterOp::Flush(ref mut fut) => {
                let result = ready!(Pin::new(fut).poll(cx));
                self.op = WriterOp::Idle;
                Poll::Ready(result.map(|_| ()).map_err(super::js_value_to_io_error))
            }
            WriterOp::Idle => {
                let fut = JsFuture::from(self.inner.ready());
                self.op = WriterOp::Flush(fut);
                self.poll_flush(cx)
            }
            _ => Poll::Pending,
        }
    }

    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        match self.op {
            WriterOp::Shutdown(ref mut fut) => {
                let result = ready!(Pin::new(fut).poll(cx));
                self.op = WriterOp::Idle;
                Poll::Ready(result.map(|_| ()).map_err(super::js_value_to_io_error))
            }
            WriterOp::Idle => {
                let fut = JsFuture::from(self.inner.close());
                self.op = WriterOp::Shutdown(fut);
                self.poll_shutdown(cx)
            }
            _ => Poll::Pending,
        }
    }
}
