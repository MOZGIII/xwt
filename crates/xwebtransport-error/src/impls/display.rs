use crate::*;

impl<Endpoint> std::fmt::Display for Connect<Endpoint>
where
    Endpoint: xwebtransport_core::EndpointConnect,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Connect::Connect(inner) => write!(f, "connect: {inner}"),
            Connect::Connecting(inner) => write!(f, "connecting: {inner}"),
        }
    }
}

impl<Endpoint> std::fmt::Display for Accept<Endpoint>
where
    Endpoint: xwebtransport_core::EndpointAccept,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Accept::Accept(inner) => write!(f, "accept: {inner}"),
            Accept::Connecting(inner) => write!(f, "connecting: {inner}"),
        }
    }
}

impl<Connect> std::fmt::Display for OpenBi<Connect>
where
    Connect: xwebtransport_core::OpenBiStream,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenBi::Open(inner) => write!(f, "open: {inner}"),
            OpenBi::Opening(inner) => write!(f, "opening: {inner}"),
        }
    }
}

impl<Connect> std::fmt::Display for OpenUni<Connect>
where
    Connect: xwebtransport_core::OpenUniStream,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenUni::Open(inner) => write!(f, "open: {inner}"),
            OpenUni::Opening(inner) => write!(f, "opening: {inner}"),
        }
    }
}
