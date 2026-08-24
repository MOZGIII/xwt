//! This test ensures that a payload that is way bigger than a single packet
//! is transferred over a stream intact and in order.
//!
//! The data is sent and read back in rounds, so that the test does not depend
//! on the flow control window being large enough to hold the whole payload.

use xwt_core::prelude::*;

/// The size of a single round of data.
const CHUNK_SIZE: usize = 4096;

/// The amount of the rounds of data to send.
const CHUNKS: usize = 16;

/// Compute the payload byte for the given offset in the stream.
///
/// The period is a prime that does not divide [`CHUNK_SIZE`], so that
/// a reordering of the data at the chunk boundaries alters the payload.
fn payload_byte(offset: usize) -> u8 {
    (offset % 251) as u8
}

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
    #[error("send: {0}")]
    Send(#[source] WriteErrorFor<SendStreamFor<ConnectSessionFor<Endpoint>>>),
    #[error("recv: {0}")]
    Recv(#[source] ReadErrorFor<RecvStreamFor<ConnectSessionFor<Endpoint>>>),
    #[error("bad data at offset {offset}")]
    BadData { offset: usize },
}

pub async fn run<Endpoint>(endpoint: Endpoint, url: &str) -> Result<(), Error<Endpoint>>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>: xwt_core::session::stream::OpenBi + std::fmt::Debug,
{
    let session = crate::utils::connect(&endpoint, url)
        .await
        .map_err(Error::Connect)?;

    let (mut send_stream, mut recv_stream) =
        crate::utils::open_bi(&session).await.map_err(Error::Open)?;

    let mut read_buf = vec![0u8; CHUNK_SIZE];

    for chunk_index in 0..CHUNKS {
        let offset = chunk_index * CHUNK_SIZE;

        let chunk = (offset..(offset + CHUNK_SIZE))
            .map(payload_byte)
            .collect::<Vec<u8>>();

        let mut to_write = &chunk[..];
        loop {
            let written = send_stream.write(to_write).await.map_err(Error::Send)?;
            let written = written.get();
            to_write = &to_write[written..];
            if to_write.is_empty() {
                break;
            }
        }

        let mut filled = 0;
        while filled < CHUNK_SIZE {
            let read = recv_stream
                .read(&mut read_buf[filled..])
                .await
                .map_err(Error::Recv)?;
            filled += read.get();
        }

        if read_buf != chunk {
            return Err(Error::BadData { offset });
        }
    }

    Ok(())
}
