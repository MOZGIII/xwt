//! A test for driving `poll_read` manually with partially-filled and
//! fully-filled read buffers.
//!
//! The `tokio::io::AsyncReadExt` helpers always pass a freshly created
//! [`tokio::io::ReadBuf`], so the `filled` portion is always empty and
//! the remaining capacity never differs from the full capacity. Callers
//! invoking [`tokio::io::AsyncRead::poll_read`] directly, however, can pass
//! a buffer that is already partially or even fully filled. This test
//! verifies the implementations honor the [`tokio::io::ReadBuf`] contract in
//! this mode:
//!
//! - a poll with no remaining capacity completes immediately without
//!   waiting for (or requesting) more stream data, and leaves the buffer
//!   contents intact;
//! - a poll with a partially-filled buffer only appends to the unfilled
//!   region, preserving the already-filled prefix;
//! - the data queued beyond the buffer remaining capacity is not lost and
//!   arrives in order on the subsequent reads.

use std::task::Poll;

use tokio::io::AsyncRead;
use xwt_core::prelude::*;

/// The sentinel byte used to detect the writes outside of the unfilled
/// region of the read buffer.
const SENTINEL: u8 = 0xAA;

#[derive(Debug, thiserror::Error)]
pub enum Error<Endpoint>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>: xwt_core::session::stream::OpenBi + std::fmt::Debug,
{
    #[error("connect: {0}")]
    Connect(#[source] xwt_error::Connect<Endpoint>),
    #[error("open: {0}")]
    Open(#[source] xwt_error::OpenBi<ConnectSessionFor<Endpoint>>),
    #[error("write: {0}")]
    Write(#[source] std::io::Error),
    #[error("read: {0}")]
    Read(#[source] std::io::Error),
    #[error("no response")]
    NoResponse,
    #[error("poll_read with a fully-filled buffer returned `Poll::Pending` instead of completing immediately")]
    FullBufPending,
    #[error("poll_read wrote outside of the unfilled region: {actual:?}")]
    ClobberedBuf { actual: Vec<u8> },
    #[error("bad data: expected {expected:?}, got {actual:?}")]
    BadData { expected: Vec<u8>, actual: Vec<u8> },
}

pub async fn run<Endpoint>(endpoint: Endpoint, url: &str) -> Result<(), Error<Endpoint>>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>: xwt_core::session::stream::OpenBi + std::fmt::Debug,
    SendStreamFor<ConnectSessionFor<Endpoint>>: tokio::io::AsyncWrite,
    RecvStreamFor<ConnectSessionFor<Endpoint>>: tokio::io::AsyncRead,
{
    let session = crate::utils::connect(&endpoint, url)
        .await
        .map_err(Error::Connect)?;

    let (send_stream, recv_stream) = crate::utils::open_bi(&session).await.map_err(Error::Open)?;

    tokio::pin!(send_stream);
    tokio::pin!(recv_stream);

    // Echo a payload and read only half of it back, so that the rest of
    // the payload is queued for reading.
    let payload = b"ABCD";
    write_all(&mut send_stream, payload).await?;
    read_exact_and_verify(recv_stream.as_mut(), &payload[..2]).await?;

    // Poll a read with a buffer that has no remaining capacity.
    // This must complete immediately - not wait for anything - and must not
    // alter the buffer, even though more stream data is queued.
    let mut array = [SENTINEL; 4];
    let mut read_buf = tokio::io::ReadBuf::new(&mut array);
    let filled = read_buf.capacity();
    read_buf.set_filled(filled);
    let poll =
        std::future::poll_fn(|cx| Poll::Ready(recv_stream.as_mut().poll_read(cx, &mut read_buf)))
            .await;
    match poll {
        Poll::Ready(Ok(())) => {}
        Poll::Ready(Err(error)) => return Err(Error::Read(error)),
        Poll::Pending => return Err(Error::FullBufPending),
    }
    if read_buf.filled() != [SENTINEL; 4] {
        return Err(Error::ClobberedBuf {
            actual: read_buf.filled().to_vec(),
        });
    }

    // The queued rest of the payload must still be readable.
    read_exact_and_verify(recv_stream.as_mut(), &payload[2..]).await?;

    // Echo another payload and read it through a partially-filled buffer:
    // only the unfilled region may be written to.
    let payload = b"EFGHIJKL";
    write_all(&mut send_stream, payload).await?;

    let mut array = [SENTINEL; 8];
    let mut read_buf = tokio::io::ReadBuf::new(&mut array);
    read_buf.set_filled(5);
    let read = std::future::poll_fn(|cx| recv_stream.as_mut().poll_read(cx, &mut read_buf))
        .await
        .map(|()| read_buf.filled().len() - 5)
        .map_err(Error::Read)?;
    if read == 0 {
        return Err(Error::NoResponse);
    }
    if read_buf.filled()[..5] != [SENTINEL; 5] {
        return Err(Error::ClobberedBuf {
            actual: read_buf.filled().to_vec(),
        });
    }
    if read_buf.filled()[5..] != payload[..read] {
        return Err(Error::BadData {
            expected: payload[..read].to_vec(),
            actual: read_buf.filled()[5..].to_vec(),
        });
    }

    // The rest of the payload - including anything the implementation may
    // have pulled off the stream beyond the buffer remaining capacity -
    // must arrive in order.
    read_exact_and_verify(recv_stream.as_mut(), &payload[read..]).await?;

    Ok(())
}

/// Write the whole payload to the stream.
async fn write_all<Endpoint, S>(
    send_stream: &mut S,
    mut to_write: &[u8],
) -> Result<(), Error<Endpoint>>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>: xwt_core::session::stream::OpenBi + std::fmt::Debug,
    S: tokio::io::AsyncWrite + Unpin,
{
    loop {
        let written = tokio::io::AsyncWriteExt::write(send_stream, to_write)
            .await
            .map_err(Error::Write)?;
        to_write = &to_write[written..];
        if to_write.is_empty() {
            return Ok(());
        }
    }
}

/// Read exactly `expected.len()` bytes from the stream and verify they match
/// the `expected` bytes.
async fn read_exact_and_verify<Endpoint, S>(
    mut recv_stream: std::pin::Pin<&mut S>,
    expected: &[u8],
) -> Result<(), Error<Endpoint>>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>: xwt_core::session::stream::OpenBi + std::fmt::Debug,
    S: tokio::io::AsyncRead,
{
    let mut received = vec![0u8; expected.len()];
    let mut filled = 0;
    while filled < expected.len() {
        let mut read_buf = tokio::io::ReadBuf::new(&mut received[filled..]);
        std::future::poll_fn(|cx| recv_stream.as_mut().poll_read(cx, &mut read_buf))
            .await
            .map_err(Error::Read)?;
        let read = read_buf.filled().len();
        if read == 0 {
            return Err(Error::NoResponse);
        }
        filled += read;
    }

    if received != expected {
        return Err(Error::BadData {
            expected: expected.to_vec(),
            actual: received,
        });
    }

    Ok(())
}
