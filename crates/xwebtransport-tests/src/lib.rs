use xwebtransport::prelude::*;

pub async fn connect<Endpoint>(
    endpoint: Endpoint,
    url: &str,
    params: Endpoint::Params<'_>,
) -> Result<(), anyhow::Error>
where
    Endpoint: xwebtransport::EndpointConnect,
    EndpointConnectConnectionFor<Endpoint>: xwebtransport::Connection,
{
    let connecting = endpoint.connect(url, params).await?;
    let connection = connecting.wait().await?;

    test_echo_on_connection(connection).await?;

    Ok(())
}

pub async fn accept<Endpoint>(endpoint: Endpoint) -> Result<(), anyhow::Error>
where
    Endpoint: xwebtransport::EndpointAccept,
    EndpointAcceptConnectionFor<Endpoint>: xwebtransport::Connection,
{
    let connecting = endpoint.accept().await?;
    let connection = connecting.wait().await?;

    test_echo_on_connection(connection).await?;

    Ok(())
}

pub async fn test_echo_on_connection<Connection>(
    connection: Connection,
) -> Result<(), anyhow::Error>
where
    Connection: xwebtransport::Connection,
{
    let opening = connection.open_bi().await?;
    let (_send, _recv) = opening.wait().await?;

    // TODO: send and receive the ping

    Ok(())
}
