//! The base WebTransport operations definitions.

use crate::session;

/// Base WebTransport session operations.
///
/// A blanket implementation for all the types that implement
/// the required base session APIs.
///
/// You might want to depend in the individual traits that this trait requires
/// ([`session::base::StreamOps`] or [`session::base::DatagramOps`]) instead in
/// your code, if you know you will only be working with the subset of the API.
///
/// Also, consider depending on the individual stream/datagram APIs directly.
pub trait Session: session::base::StreamOps + session::base::DatagramOps {}

impl<T> Session for T where T: session::base::StreamOps + session::base::DatagramOps {}
