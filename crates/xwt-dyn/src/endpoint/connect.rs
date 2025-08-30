#![allow(missing_docs)]

use alloc::boxed::Box;
use xwt_core::utils::maybe;

use crate::utils::traits::{maybe_send, maybe_send_sync};

#[dyn_safe::dyn_safe(true)]
pub trait Connect: maybe::Send + maybe::Sync {
    fn connect<'a>(
        &'a self,
        url: &'a str,
    ) -> maybe_send::BoxedFuture<
        'a,
        Result<Box<dyn Connecting + 'static>, maybe_send_sync::BoxedError<'static>>,
    >;
}

impl<X> Connect for X
where
    X: xwt_core::endpoint::Connect,
    X: maybe::Sync,
    X: 'static,
    xwt_core::prelude::ConnectSessionFor<X>: crate::base::Session,
{
    fn connect<'a>(
        &'a self,
        url: &'a str,
    ) -> maybe_send::BoxedFuture<
        'a,
        Result<Box<dyn Connecting + 'static>, maybe_send_sync::BoxedError<'static>>,
    > {
        Box::pin(async move {
            <X as xwt_core::endpoint::Connect>::connect(self, url)
                .await
                .map(|val| Box::new(val) as _)
                .map_err(|error| Box::new(error) as _)
        })
    }
}

#[dyn_safe::dyn_safe(true)]
pub trait Connecting: maybe::Send {
    fn wait_connect(
        self: Box<Self>,
    ) -> maybe_send::BoxedFuture<
        'static,
        Result<Box<dyn crate::base::Session + 'static>, maybe_send_sync::BoxedError<'static>>,
    >;
}

impl<X> Connecting for X
where
    X: xwt_core::endpoint::connect::Connecting,
    X: 'static,
    xwt_core::prelude::ConnectingSessionFor<X>: crate::base::Session,
{
    fn wait_connect(
        self: Box<Self>,
    ) -> maybe_send::BoxedFuture<
        'static,
        Result<Box<dyn crate::base::Session + 'static>, maybe_send_sync::BoxedError<'static>>,
    > {
        Box::pin(async move {
            <X as xwt_core::endpoint::connect::Connecting>::wait_connect(*self)
                .await
                .map(|val| Box::new(val) as _)
                .map_err(|error| Box::new(error) as _)
        })
    }
}
