//! The base WebTransport session operations definition.

use crate::utils::maybe;

use super::{datagram, stream};

/// Base stream operations.
///
/// A blanket implementation for all the types that implement
/// the required base stream APIs.
///
/// You might want to depend on this trait instead of the more
/// demaning [`crate::base::Session`] in your libraries that only require
/// stream operations.
///
/// Also, consider depending on the individual stream APIs directly.
pub trait StreamOps:
    stream::PairSpec + stream::OpenBi + stream::OpenUni + stream::AcceptBi + stream::AcceptUni
{
}

impl<T> StreamOps for T where
    T: stream::PairSpec + stream::OpenBi + stream::OpenUni + stream::AcceptBi + stream::AcceptUni
{
}

/// Base datagram operations.
///
/// A blanket implementation for all the types that implement
/// the required base datagram APIs.
///
/// You might want to depend on this trait instead of the more
/// demaning [`crate::base::Session`] in your libraries that only require
/// stream operations.
///
/// Also, consider depending on the individual datagram APIs directly.
pub trait DatagramOps:
    maybe::Send + datagram::MaxSize + datagram::Send + datagram::Receive + datagram::ReceiveInto
{
}

impl<T> DatagramOps for T where
    T: maybe::Send + datagram::MaxSize + datagram::Send + datagram::Receive + datagram::ReceiveInto
{
}
