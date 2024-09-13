use xwt_core::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum Error<Endpoint>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>: xwt_core::session::stream::AcceptBi + std::fmt::Debug,
{
    #[error("connect: {0}")]
    Connect(#[source] xwt_error::Connect<Endpoint>),
    #[error("accept bi stream: {0}")]
    AcceptBiStream(
        #[source] <ConnectSessionFor<Endpoint> as xwt_core::session::stream::AcceptBi>::Error,
    ),
    #[error("send: {0}")]
    Send(#[source] WriteErrorFor<SendStreamFor<ConnectSessionFor<Endpoint>>>),
    #[error("recv: {0}")]
    Recv(#[source] ReadErrorFor<RecvStreamFor<ConnectSessionFor<Endpoint>>>),
    #[error("bad data")]
    BadData(Vec<u8>),
}

pub async fn run<Endpoint>(endpoint: Endpoint, url: &str) -> Result<(), Error<Endpoint>>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>: xwt_core::session::stream::AcceptBi + std::fmt::Debug,
{
    let session = crate::utils::connect(&endpoint, url)
        .await
        .map_err(Error::Connect)?;

    let (mut send_stream, mut recv_stream) =
        session.accept_bi().await.map_err(Error::AcceptBiStream)?;

    let mut to_write = &b"hello"[..];
    loop {
        let written = send_stream.write(to_write).await.map_err(Error::Send)?;
        let written = written.get();
        to_write = &to_write[written..];
        if to_write.is_empty() {
            break;
        }
    }

    let mut read_buf = vec![0u8; 1024];

    let read = recv_stream
        .read(&mut read_buf[..])
        .await
        .map_err(Error::Recv)?;
    let read = read.get();
    read_buf.truncate(read);

    if read_buf != b"hello" {
        return Err(Error::BadData(read_buf));
    }

    Ok(())
}
