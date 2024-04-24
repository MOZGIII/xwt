use crate::*;

impl<Endpoint> std::error::Error for Connect<Endpoint> where Endpoint: xwt_core::endpoint::Connect {}

impl<Endpoint> std::error::Error for Accept<Endpoint> where Endpoint: xwt_core::endpoint::Accept {}

impl<TAccepting> std::error::Error for Accepting<TAccepting> where
    TAccepting: xwt_core::endpoint::accept::Accepting
{
}

impl<Session> std::error::Error for OpenBi<Session> where Session: xwt_core::session::stream::OpenBi {}

impl<Session> std::error::Error for OpenUni<Session> where
    Session: xwt_core::session::stream::OpenUni
{
}
