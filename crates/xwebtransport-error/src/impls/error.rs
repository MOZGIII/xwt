use crate::*;

impl<Endpoint> std::error::Error for Connect<Endpoint> where
    Endpoint: xwebtransport_core::EndpointConnect
{
}

impl<Endpoint> std::error::Error for Accept<Endpoint> where
    Endpoint: xwebtransport_core::EndpointAccept
{
}

impl<TAccepting> std::error::Error for Accepting<TAccepting> where
    TAccepting: xwebtransport_core::Accepting
{
}

impl<Connect> std::error::Error for OpenBi<Connect> where Connect: xwebtransport_core::OpenBiStream {}

impl<Connect> std::error::Error for OpenUni<Connect> where Connect: xwebtransport_core::OpenUniStream
{}
