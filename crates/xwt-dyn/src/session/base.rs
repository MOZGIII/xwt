//! The base WebTransport session operations definition.

use xwt_core::utils::maybe;

use super::{datagram, stream};

#[dyn_safe::dyn_safe(true)]
pub trait StreamOps:
    stream::OpenBi + stream::OpenUni + stream::AcceptBi + stream::AcceptUni
{
}

impl<T> StreamOps for T where
    T: stream::OpenBi + stream::OpenUni + stream::AcceptBi + stream::AcceptUni
{
}

#[dyn_safe::dyn_safe(true)]
pub trait DatagramOps:
    maybe::Send
    + xwt_core::session::datagram::MaxSize
    + datagram::Send
    + datagram::Receive
    + datagram::ReceiveInto
{
}

impl<T> DatagramOps for T where
    T: maybe::Send
        + xwt_core::session::datagram::MaxSize
        + datagram::Send
        + datagram::Receive
        + datagram::ReceiveInto
{
}
