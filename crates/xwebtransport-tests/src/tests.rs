#[derive(Debug, thiserror::Error)]
pub enum EchoError<Connection>
where
    Connection: xwebtransport_core::OpenBiStream,
{
    Open(xwebtransport_error::OpenBi<Connection>),
}

pub async fn echo<Connection>(connection: Connection) -> Result<(), EchoError<Connection>>
where
    Connection: xwebtransport_core::Connection,
{
    let (_send, _recv) = crate::utils::open_bi(connection)
        .await
        .map_err(EchoError::Open)?;

    // TODO: send and receive the ping

    Ok(())
}
