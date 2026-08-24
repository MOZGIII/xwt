//! A test for reading a large echoed payload through a read buffer that is
//! smaller than the read buffers used before.
//!
//! Implementations that keep an internal read buffer sized after the largest
//! read so far can end up with a single received data chunk that is bigger
//! than the buffer the caller passes to `read`, and thus have to hand out
//! the chunk across multiple consecutive reads. This test verifies the data
//! and the order of the bytes are preserved in this mode - which requires,
//! in particular, correctly tracking the resume offset within the chunk
//! across three or more reads.

use xwt_core::prelude::*;

/// The size of the large payloads and the large read buffer.
///
/// Must be big enough that consuming a single chunk of this size via
/// the [`SMALL_BUF_SIZE`] buffer takes three or more reads.
const LARGE_BUF_SIZE: usize = 64;

/// The size of the small read buffer.
///
/// Intentionally not a divisor of [`LARGE_BUF_SIZE`] to also exercise
/// a trailing partial read.
const SMALL_BUF_SIZE: usize = 3;

/// The amount of large payloads to drain through the small read buffer.
///
/// More rounds increase the chance of observing a large queued chunk even
/// when the network fragments some of the payloads.
const SMALL_READ_ROUNDS: usize = 3;

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

    // Echo a large payload and read it back with an equally large buffer.
    // This seeds the implementations that retain an internal read buffer
    // with a buffer of this (large) size.
    let payload = payload_for_round(0);
    write_all(&mut send_stream, &payload).await?;
    read_and_verify(&mut recv_stream, LARGE_BUF_SIZE, &payload).await?;

    // Echo large payloads again, but read them back through a small buffer.
    // By the time we read, a whole payload is typically queued, so a single
    // received chunk has to be consumed across many small reads.
    for round in 1..=SMALL_READ_ROUNDS {
        let payload = payload_for_round(round);
        write_all(&mut send_stream, &payload).await?;
        read_and_verify(&mut recv_stream, SMALL_BUF_SIZE, &payload).await?;
    }

    Ok(())
}

/// Generate a payload of [`LARGE_BUF_SIZE`] bytes distinct across all rounds,
/// so that misplaced or repeated bytes are always detected.
fn payload_for_round(round: usize) -> Vec<u8> {
    (0..LARGE_BUF_SIZE)
        .map(|i| u8::try_from(round * LARGE_BUF_SIZE + i).unwrap())
        .collect()
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

/// Read `expected.len()` bytes from the stream using a read buffer of
/// the given size and verify the read data matches the `expected` payload
/// exactly.
async fn read_and_verify<Endpoint, S>(
    recv_stream: &mut S,
    read_buf_size: usize,
    expected: &[u8],
) -> Result<(), Error<Endpoint>>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>: xwt_core::session::stream::OpenBi + std::fmt::Debug,
    S: tokio::io::AsyncRead + Unpin,
{
    let mut read_buf = vec![0u8; read_buf_size];
    let mut received = Vec::with_capacity(expected.len());
    while received.len() < expected.len() {
        let read = tokio::io::AsyncReadExt::read(recv_stream, &mut read_buf[..])
            .await
            .map_err(Error::Read)?;
        if read == 0 {
            return Err(Error::NoResponse);
        }
        received.extend_from_slice(&read_buf[..read]);
    }

    if received != expected {
        return Err(Error::BadData {
            expected: expected.to_vec(),
            actual: received,
        });
    }

    Ok(())
}
