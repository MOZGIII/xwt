#![allow(missing_docs)]

use alloc::boxed::Box;
use xwt_core::utils::maybe;

use crate::utils::traits::{maybe_send, maybe_send_sync};

#[dyn_safe::dyn_safe(true)]
pub trait IncomingDatagram: maybe::Send + AsRef<[u8]> {}

impl<X> IncomingDatagram for X where X: maybe::Send + AsRef<[u8]> {}

#[dyn_safe::dyn_safe(true)]
pub trait Receive: maybe::Send {
    fn receive_datagram(
        &self,
    ) -> maybe_send::BoxedFuture<
        '_,
        Result<Box<dyn IncomingDatagram>, maybe_send_sync::BoxedError<'static>>,
    >;
}

impl<X> Receive for X
where
    X: xwt_core::session::datagram::Receive,
    X: 'static,
    X: maybe::Sync,
{
    fn receive_datagram(
        &self,
    ) -> maybe_send::BoxedFuture<
        '_,
        Result<Box<dyn IncomingDatagram>, maybe_send_sync::BoxedError<'static>>,
    > {
        Box::pin(async move {
            <X as xwt_core::session::datagram::Receive>::receive_datagram(self)
                .await
                .map(|val| Box::new(val) as _)
                .map_err(|error| Box::new(error) as _)
        })
    }
}

#[dyn_safe::dyn_safe(true)]
pub trait ReceiveInto: maybe::Send {
    fn receive_datagram_into<'a>(
        &'a self,
        buf: &'a mut [u8],
    ) -> maybe_send::BoxedFuture<'a, Result<usize, maybe_send_sync::BoxedError<'static>>>;
}

impl<X> ReceiveInto for X
where
    X: xwt_core::session::datagram::ReceiveInto,
    X: 'static,
    X: maybe::Sync,
{
    fn receive_datagram_into<'a>(
        &'a self,
        buf: &'a mut [u8],
    ) -> maybe_send::BoxedFuture<'a, Result<usize, maybe_send_sync::BoxedError<'static>>> {
        Box::pin(async move {
            <X as xwt_core::session::datagram::ReceiveInto>::receive_datagram_into(self, buf)
                .await
                .map_err(|error| Box::new(error) as _)
        })
    }
}

#[dyn_safe::dyn_safe(true)]
pub trait Send: maybe::Send {
    fn send_datagram<'a>(
        &'a self,
        payload: &'a [u8],
    ) -> maybe_send::BoxedFuture<'a, Result<(), maybe_send_sync::BoxedError<'static>>>;
}

impl<X> Send for X
where
    X: xwt_core::session::datagram::Send,
    X: 'static,
    X: maybe::Sync,
{
    fn send_datagram<'a>(
        &'a self,
        payload: &'a [u8],
    ) -> maybe_send::BoxedFuture<'a, Result<(), maybe_send_sync::BoxedError<'static>>> {
        Box::pin(async move {
            <X as xwt_core::session::datagram::Send>::send_datagram(self, payload)
                .await
                .map_err(|error| Box::new(error) as _)
        })
    }
}
