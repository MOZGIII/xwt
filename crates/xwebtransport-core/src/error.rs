use crate::prelude::*;

pub enum ConnectError<Endpoint>
where
    Endpoint: crate::traits::EndpointConnect,
    EndpointConnectConnectionFor<Endpoint>: crate::traits::Connection,
{
    Connect(Endpoint::Error),
    Connecting(<Endpoint::Connecting as crate::traits::Connecting>::Error),
}

pub enum AcceptError<Endpoint>
where
    Endpoint: crate::traits::EndpointAccept,
    EndpointAcceptConnectionFor<Endpoint>: crate::traits::Connection,
{
    Accept(Endpoint::Error),
    Connecting(<Endpoint::Connecting as crate::traits::Connecting>::Error),
}

pub enum OpenBiStreamError<Connection>
where
    Connection: crate::traits::Connection,
{
    Open(<Connection as crate::traits::OpenBiStream>::Error),
    Opening(BiStreamOpeningErrorFor<Connection>),
}

pub enum OpenUniStreamError<Connection>
where
    Connection: crate::traits::Connection,
{
    Open(<Connection as crate::traits::OpenUniStream>::Error),
    Opening(UniStreamOpeningErrorFor<Connection>),
}
