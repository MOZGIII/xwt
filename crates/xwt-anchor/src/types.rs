//! This crate's types.
//!
//! These types are generic around the underlying implementaion that is
//! typically required to implement corresponding [`xwt_core`] traits.

/// A WebTransport endpoint.
#[repr(transparent)]
pub struct Endpoint<T>(pub T);

/// A state of a connecting WebTransport connection.
#[repr(transparent)]
pub struct Connecting<T>(pub T);

/// A state of a accepting WebTransport connection.
#[repr(transparent)]
pub struct Accepting<T>(pub T);

/// An incoming WebTransport session request.
#[repr(transparent)]
pub struct Request<T>(pub T);

/// A WebTransport connection.
#[repr(transparent)]
pub struct Connection<T>(pub T);

/// An state for an opening bidirectional WebTransport stream.
#[repr(transparent)]
pub struct OpeningBiStream<T>(pub T);

/// An state for an opening unidirectional WebTransport stream.
#[repr(transparent)]
pub struct OpeningUniStream<T>(pub T);

/// A WebTransport send-stream.
#[repr(transparent)]
pub struct SendStream<T>(pub T);

/// A WebTransport recv-stream.
#[repr(transparent)]
pub struct RecvStream<T>(pub T);

/// A pair of WebTransport streams.
///
/// Never actually instantiated.
#[repr(transparent)]
pub struct Streams<T>((core::marker::PhantomData<T>, core::convert::Infallible));

/// A WebTransport datagram.
#[repr(transparent)]
pub struct Datagram<T>(pub T);
