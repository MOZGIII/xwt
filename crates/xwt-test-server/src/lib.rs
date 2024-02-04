//! The implementation of the echo server to use for xwt testing.
//! Not intended to be wasm-compatible.

#![allow(missing_docs)]
#![cfg(not(target_family = "wasm"))]

use std::sync::Arc;

#[derive(Default)]
pub struct EndpointParams {
    pub addr: Option<std::net::SocketAddr>,
    pub cert: Option<wtransport::tls::Certificate>,
}

pub async fn endpoint(
    params: EndpointParams,
) -> Result<wtransport::Endpoint<wtransport::endpoint::endpoint_side::Server>, std::io::Error> {
    let EndpointParams { addr, cert } = params;

    let cert = cert.unwrap_or_else(|| {
        wtransport::tls::Certificate::new(vec![xwt_test_assets::CERT], xwt_test_assets::KEY)
            .unwrap()
    });

    match cert.certificates().first() {
        Some(cert) => {
            let sha256_fingerpint = xwt_cert_utils::fingerprint::sha256(cert);
            tracing::info!(message = "using tls certificate", %sha256_fingerpint);
        }
        None => tracing::info!(message = "not using tls certificate"),
    };

    let server_config =
        wtransport::ServerConfig::builder()
            .with_bind_address(addr.unwrap_or(std::net::SocketAddr::V4(
                std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), 0),
            )))
            .with_certificate(cert)
            .build();

    let endpoint = wtransport::Endpoint::server(server_config)?;

    Ok(endpoint)
}

pub async fn serve_endpoint(
    endpoint: wtransport::Endpoint<wtransport::endpoint::endpoint_side::Server>,
) -> Result<std::convert::Infallible, std::io::Error> {
    let bind_addr = endpoint.local_addr()?;
    tracing::info!(message = "serving endpoint", %bind_addr);

    loop {
        let incoming_session = endpoint.accept().await;
        tokio::spawn(async move {
            if let Err(error) = serve_incoming_session(incoming_session).await {
                tracing::error!(message = "error while serving incoming session", %error);
            }
        });
    }
}

pub async fn serve_incoming_session(
    incoming_session: wtransport::endpoint::IncomingSession,
) -> Result<(), wtransport::error::ConnectionError> {
    tracing::info!(message = "got an incoming session");
    let session_request = incoming_session.await?;

    if session_request.path() != "/echo" {
        tracing::info!(message = "rejecting incoming session due to path mismatch");
        session_request.not_found().await;
        return Ok(());
    }

    tracing::info!(message = "accepting incoming session");
    let connection = session_request.accept().await?;

    tracing::info!(message = "new connection accepted");

    let connection = Arc::new(connection);

    let mut joinset = tokio::task::JoinSet::new();

    {
        let connection = Arc::clone(&connection);
        joinset.spawn(async move {
            tracing::info!(message = "serving streams");
            if let Err(error) = serve_streams(connection).await {
                tracing::error!(message = "error while serving streams", %error);
            }
            tracing::info!(message = "done serving streams");
        });
    }
    {
        let connection = Arc::clone(&connection);
        joinset.spawn(async move {
            tracing::info!(message = "serving datagrams");
            if let Err(error) = serve_datagrams(connection).await {
                tracing::error!(message = "error while serving datagrams", %error);
            }
            tracing::info!(message = "done serving datagrams");
        });
    }

    connection.closed().await;

    tracing::info!(message = "connection is closing");

    while let Some(result) = joinset.join_next().await {
        if let Err(panic) = result {
            tracing::error!(message = "panic in the connection task", %panic);
        }
    }

    tracing::info!(message = "connection tasks are finished");

    Ok(())
}

pub async fn serve_streams(
    connection: impl AsRef<wtransport::Connection>,
) -> Result<std::convert::Infallible, wtransport::error::ConnectionError> {
    let connection = connection.as_ref();
    loop {
        let stream = connection.accept_bi().await?;
        tokio::spawn(async move {
            tracing::info!(message = "serving stream");
            if let Err(error) = serve_stream(stream).await {
                tracing::error!(message = "error while serving stream", %error);
            }
            tracing::info!(message = "done serving stream");
        });
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StreamError {
    #[error("read: {0}")]
    Read(wtransport::error::StreamReadError),
    #[error("write: {0}")]
    Write(wtransport::error::StreamWriteError),
}

pub async fn serve_stream(
    stream: (wtransport::SendStream, wtransport::RecvStream),
) -> Result<(), StreamError> {
    let (mut tx, mut rx) = stream;
    let mut buf = vec![0; 1024];
    loop {
        let Some(len) = rx.read(&mut buf).await.map_err(StreamError::Read)? else {
            tracing::info!(message = "stream closed");
            return Ok(());
        };
        tracing::info!(message = "read stream data", %len);
        tx.write_all(&buf[..len])
            .await
            .map_err(StreamError::Write)?;
        tracing::info!(message = "written stream data", %len);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DatagramError {
    #[error("receive: {0}")]
    Receive(wtransport::error::ConnectionError),
    #[error("send: {0}")]
    Send(wtransport::error::SendDatagramError),
}

pub async fn serve_datagrams(
    connection: impl AsRef<wtransport::Connection>,
) -> Result<(), DatagramError> {
    let connection = connection.as_ref();
    loop {
        let datagram = connection
            .receive_datagram()
            .await
            .map_err(DatagramError::Receive)?;
        let size = datagram.payload().len();
        tracing::info!(message = "received datagram", %size);
        connection
            .send_datagram(datagram.payload())
            .map_err(DatagramError::Send)?;
        tracing::info!(message = "written datagram", %size);
    }
}
