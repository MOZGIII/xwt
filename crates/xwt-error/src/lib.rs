//! A suite of reusable error types that naturally emerge from the `xwt` API.
//! Use when you don't want/need your own more precise types.

#![allow(missing_docs, clippy::missing_docs_in_private_items)]

mod impls;

use xwt_core::prelude::*;

pub enum Connect<Endpoint>
where
    Endpoint: xwt_core::endpoint::Connect,
{
    Connect(Endpoint::Error),
    Connecting(<Endpoint::Connecting as xwt_core::endpoint::connect::Connecting>::Error),
}

pub enum Accept<Endpoint>
where
    Endpoint: xwt_core::endpoint::Accept,
{
    Accept(Endpoint::Error),
}

pub enum Accepting<TAccepting>
where
    TAccepting: xwt_core::endpoint::accept::Accepting,
{
    Accepting(TAccepting::Error),
    RequestOk(<TAccepting::Request as xwt_core::endpoint::accept::Request>::OkError),
    RequestClose(<TAccepting::Request as xwt_core::endpoint::accept::Request>::CloseError),
}

pub enum OpenBi<Session>
where
    Session: xwt_core::session::stream::OpenBi,
{
    Open(<Session as xwt_core::session::stream::OpenBi>::Error),
    Opening(BiStreamOpeningErrorFor<Session>),
}

pub enum OpenUni<Session>
where
    Session: xwt_core::session::stream::OpenUni,
{
    Open(<Session as xwt_core::session::stream::OpenUni>::Error),
    Opening(UniStreamOpeningErrorFor<Session>),
}
