use xwebtransport_core::{prelude::*, traits};

#[derive(Debug, thiserror::Error)]
pub enum Connect<Endpoint>
where
    Endpoint: traits::EndpointConnect,
{
    Connect(Endpoint::Error),
    Connecting(<Endpoint::Connecting as traits::Connecting>::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum Accept<Endpoint>
where
    Endpoint: traits::EndpointAccept,
{
    Accept(Endpoint::Error),
    Connecting(<Endpoint::Connecting as traits::Connecting>::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum OpenBi<Connection>
where
    Connection: traits::OpenBiStream,
{
    Open(<Connection as traits::OpenBiStream>::Error),
    Opening(BiStreamOpeningErrorFor<Connection>),
}

#[derive(Debug, thiserror::Error)]
pub enum OpenUni<Connection>
where
    Connection: traits::OpenUniStream,
{
    Open(<Connection as traits::OpenUniStream>::Error),
    Opening(UniStreamOpeningErrorFor<Connection>),
}
