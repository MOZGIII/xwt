use xwt_core::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum Error<Endpoint, WriteChunk, ReadChunk>
where
    Endpoint: xwt_core::EndpointConnect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    EndpointConnectConnectionFor<Endpoint>: xwt_core::OpenBiStream + std::fmt::Debug,

    WriteChunk: xwt_core::WriteableChunk + std::fmt::Debug,
    ReadChunk: xwt_core::ReadableChunk + std::fmt::Debug,

    SendStreamFor<EndpointConnectConnectionFor<Endpoint>>: xwt_core::WriteChunk<WriteChunk>,
    RecvStreamFor<EndpointConnectConnectionFor<Endpoint>>: xwt_core::ReadChunk<ReadChunk>,
{
    #[error("connect: {0}")]
    Connect(#[source] xwt_error::Connect<Endpoint>),
    #[error("open: {0}")]
    Open(#[source] xwt_error::OpenBi<EndpointConnectConnectionFor<Endpoint>>),
    #[error("send: {0}")]
    Send(
        #[source]
        WriteChunkErrorFor<SendStreamFor<EndpointConnectConnectionFor<Endpoint>>, WriteChunk>,
    ),
    #[error("recv: {0}")]
    Recv(
        #[source]
        ReadChunkErrorFor<RecvStreamFor<EndpointConnectConnectionFor<Endpoint>>, ReadChunk>,
    ),
    #[error("no response")]
    NoResponse,
    #[error("bad data")]
    BadData(Vec<u8>),
}

pub async fn run<Endpoint, WriteChunk, ReadChunk>(
    endpoint: Endpoint,
    url: &str,
) -> Result<(), Error<Endpoint, WriteChunk, ReadChunk>>
where
    Endpoint: xwt_core::EndpointConnect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    EndpointConnectConnectionFor<Endpoint>: xwt_core::OpenBiStream + std::fmt::Debug,

    WriteChunk: xwt_core::WriteableChunk + std::fmt::Debug,
    ReadChunk: xwt_core::ReadableChunk + std::fmt::Debug,

    <WriteChunk as xwt_core::WriteableChunk>::Data<'static>: From<&'static [u8]>,
    for<'a> <ReadChunk as xwt_core::ReadableChunk>::Data<'a>: AsRef<[u8]>,

    SendStreamFor<EndpointConnectConnectionFor<Endpoint>>: xwt_core::WriteChunk<WriteChunk>,
    RecvStreamFor<EndpointConnectConnectionFor<Endpoint>>: xwt_core::ReadChunk<ReadChunk>,
{
    let connection = crate::utils::connect(&endpoint, url)
        .await
        .map_err(Error::Connect)?;

    let (mut send_stream, mut recv_stream) = crate::utils::open_bi(&connection)
        .await
        .map_err(Error::Open)?;

    let write_data: WriteChunk::Data<'static> = (&b"hello"[..]).into();
    send_stream
        .write_chunk(write_data)
        .await
        .map_err(Error::Send)?;

    let maybe_read_chunk = recv_stream
        .read_chunk(1024, false)
        .await
        .map_err(Error::Recv)?;
    let Some(read_chunk) = maybe_read_chunk else {
        return Err(Error::NoResponse);
    };
    let read_data = read_chunk.data.as_ref();

    if read_data != b"hello" {
        return Err(Error::BadData(read_data.into()));
    }

    Ok(())
}
