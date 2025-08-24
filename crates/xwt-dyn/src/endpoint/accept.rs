#![allow(missing_docs)]

use alloc::boxed::Box;
use xwt_core::utils::maybe;

use crate::utils::traits::{maybe_send, maybe_send_sync};

#[dyn_safe::dyn_safe(true)]
pub trait Accept: maybe::Send + maybe::Sync {
    fn accept(
        &self,
    ) -> maybe_send::BoxedFuture<
        '_,
        Result<Option<Box<dyn Accepting + 'static>>, maybe_send_sync::BoxedError<'static>>,
    >;
}

impl<X> Accept for X
where
    X: xwt_core::endpoint::Accept,
    X: maybe::Sync,
    X: 'static,
    xwt_core::prelude::AcceptRequestFor<X>: maybe::Sync,
    xwt_core::prelude::AcceptSessionFor<X>: crate::base::Session,
{
    fn accept(
        &self,
    ) -> maybe_send::BoxedFuture<
        '_,
        Result<Option<Box<dyn Accepting + 'static>>, maybe_send_sync::BoxedError<'static>>,
    > {
        Box::pin(async move {
            <X as xwt_core::endpoint::Accept>::accept(self)
                .await
                .map(|val| val.map(|val| Box::new(val) as _))
                .map_err(|error| Box::new(error) as _)
        })
    }
}

#[dyn_safe::dyn_safe(true)]
pub trait Accepting: maybe::Send {
    fn wait_accept(
        self: Box<Self>,
    ) -> maybe_send::BoxedFuture<
        'static,
        Result<Box<dyn Request + 'static>, maybe_send_sync::BoxedError<'static>>,
    >;
}

impl<X> Accepting for X
where
    X: xwt_core::endpoint::accept::Accepting<Request: maybe::Sync>,
    X: 'static,
    xwt_core::prelude::AcceptingSessionFor<X>: crate::base::Session,
{
    fn wait_accept(
        self: Box<Self>,
    ) -> maybe_send::BoxedFuture<
        'static,
        Result<Box<dyn Request + 'static>, maybe_send_sync::BoxedError<'static>>,
    > {
        Box::pin(async move {
            <X as xwt_core::endpoint::accept::Accepting>::wait_accept(*self)
                .await
                .map(|val| Box::new(val) as _)
                .map_err(|error| Box::new(error) as _)
        })
    }
}

#[dyn_safe::dyn_safe(true)]
pub trait Request: maybe::Send + maybe::Sync {
    fn ok(
        self: Box<Self>,
    ) -> maybe_send::BoxedFuture<
        'static,
        Result<Box<dyn crate::base::Session + 'static>, maybe_send_sync::BoxedError<'static>>,
    >;

    fn close(
        self: Box<Self>,
        status: u16,
    ) -> maybe_send::BoxedFuture<'static, Result<(), maybe_send_sync::BoxedError<'static>>>;
}

impl<X> Request for X
where
    X: xwt_core::endpoint::accept::Request,
    X: maybe::Sync,
    X: 'static,
    xwt_core::prelude::RequestSessionFor<X>: crate::base::Session,
{
    fn ok(
        self: Box<Self>,
    ) -> maybe_send::BoxedFuture<
        'static,
        Result<Box<dyn crate::base::Session + 'static>, maybe_send_sync::BoxedError<'static>>,
    > {
        Box::pin(async move {
            <X as xwt_core::endpoint::accept::Request>::ok(*self)
                .await
                .map(|val| Box::new(val) as _)
                .map_err(|error| Box::new(error) as _)
        })
    }

    fn close(
        self: Box<Self>,
        status: u16,
    ) -> maybe_send::BoxedFuture<'static, Result<(), maybe_send_sync::BoxedError<'static>>> {
        Box::pin(async move {
            <X as xwt_core::endpoint::accept::Request>::close(*self, status)
                .await
                .map_err(|error| Box::new(error) as _)
        })
    }
}
