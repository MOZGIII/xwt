//! This test ensures that multiple streams can be open on a single session
//! at the same time, and that the data on them is not mixed up.

use xwt_core::prelude::*;

/// The amount of the streams to open.
const STREAMS: usize = 4;

/// Compute the payload to send over the stream with the given index.
///
/// The payloads have different lengths, so that a stream that responds with
/// the data of another stream is detected even if the reads are aligned.
fn payload(index: usize) -> Vec<u8> {
    let mut payload = format!("stream-{index}").into_bytes();
    payload.extend(std::iter::repeat_n(b'.', index));
    payload
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
    #[error("bad data at stream {index}")]
    BadData { index: usize, data: Vec<u8> },
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

    let mut streams = Vec::with_capacity(STREAMS);

    for index in 0..STREAMS {
        let (mut send_stream, recv_stream) =
            crate::utils::open_bi(&session).await.map_err(Error::Open)?;

        let expected = payload(index);

        let mut to_write = &expected[..];
        loop {
            let written = send_stream.write(to_write).await.map_err(Error::Send)?;
            let written = written.get();
            to_write = &to_write[written..];
            if to_write.is_empty() {
                break;
            }
        }

        // The send streams are kept around until the end of the test, as
        // dropping them can abort the whole stream.
        streams.push((index, send_stream, recv_stream, expected));
    }

    // Read the responses in the reverse order to ensure the streams are
    // independent of one another rather than being a single queue.
    for (index, _send_stream, mut recv_stream, expected) in streams.into_iter().rev() {
        let mut read_buf = vec![0u8; expected.len()];

        let mut filled = 0;
        while filled < expected.len() {
            let read = recv_stream
                .read(&mut read_buf[filled..])
                .await
                .map_err(Error::Recv)?;
            filled += read.get();
        }

        if read_buf != expected {
            return Err(Error::BadData {
                index,
                data: read_buf,
            });
        }
    }

    Ok(())
}
