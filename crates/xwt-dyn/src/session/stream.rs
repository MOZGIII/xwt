#![allow(missing_docs)]

use alloc::boxed::Box;
use xwt_core::utils::maybe;

use crate::utils::traits::{maybe_send, maybe_send_sync};

#[dyn_safe::dyn_safe(true)]
pub trait SendStream:
    crate::stream::Write
    + crate::stream::WriteAbort
    + crate::stream::WriteAborted
    + crate::stream::Finish
{
}

impl<X> SendStream for X where
    X: crate::stream::Write
        + crate::stream::WriteAbort
        + crate::stream::WriteAborted
        + crate::stream::Finish
{
}

#[dyn_safe::dyn_safe(true)]
pub trait RecvStream: crate::stream::Read + crate::stream::ReadAbort {}

impl<X> RecvStream for X where X: crate::stream::Read + crate::stream::ReadAbort {}

pub type StreamsTuple<'a> = (Box<dyn SendStream + 'a>, Box<dyn RecvStream + 'a>);

#[dyn_safe::dyn_safe(true)]
pub trait OpeningBi: maybe::Send {
    fn wait_bi(
        self: Box<Self>,
    ) -> maybe_send::BoxedFuture<
        'static,
        Result<StreamsTuple<'static>, maybe_send_sync::BoxedError<'static>>,
    >;
}

impl<X> OpeningBi for X
where
    X: xwt_core::session::stream::OpeningBi,
    X: 'static,
{
    fn wait_bi(
        self: Box<Self>,
    ) -> maybe_send::BoxedFuture<
        'static,
        Result<StreamsTuple<'static>, maybe_send_sync::BoxedError<'static>>,
    > {
        Box::pin(async move {
            <X as xwt_core::session::stream::OpeningBi>::wait_bi(*self)
                .await
                .map(|(tx, rx)| (Box::new(tx) as _, Box::new(rx) as _))
                .map_err(|error| Box::new(error) as _)
        })
    }
}

#[dyn_safe::dyn_safe(true)]
pub trait OpenBi: maybe::Send + maybe::Sync {
    fn open_bi(
        &self,
    ) -> maybe_send::BoxedFuture<
        '_,
        Result<Box<dyn OpeningBi + 'static>, maybe_send_sync::BoxedError<'static>>,
    >;
}

impl<X> OpenBi for X
where
    X: xwt_core::session::stream::OpenBi,
    X: maybe::Sync,
    X: 'static,
{
    fn open_bi(
        &self,
    ) -> maybe_send::BoxedFuture<
        '_,
        Result<Box<dyn OpeningBi + 'static>, maybe_send_sync::BoxedError<'static>>,
    > {
        Box::pin(async move {
            <X as xwt_core::session::stream::OpenBi>::open_bi(self)
                .await
                .map(|val| Box::new(val) as _)
                .map_err(|error| Box::new(error) as _)
        })
    }
}

#[dyn_safe::dyn_safe(true)]
pub trait AcceptBi: maybe::Send {
    fn accept_bi(
        &self,
    ) -> maybe_send::BoxedFuture<
        '_,
        Result<StreamsTuple<'static>, maybe_send_sync::BoxedError<'static>>,
    >;
}

impl<X> AcceptBi for X
where
    X: xwt_core::session::stream::AcceptBi,
    X: maybe::Sync,
    X: 'static,
{
    fn accept_bi(
        &self,
    ) -> maybe_send::BoxedFuture<
        '_,
        Result<StreamsTuple<'static>, maybe_send_sync::BoxedError<'static>>,
    > {
        Box::pin(async move {
            <X as xwt_core::session::stream::AcceptBi>::accept_bi(self)
                .await
                .map(|(tx, rx)| (Box::new(tx) as _, Box::new(rx) as _))
                .map_err(|error| Box::new(error) as _)
        })
    }
}

#[dyn_safe::dyn_safe(true)]
pub trait OpeningUni: maybe::Send {
    fn wait_uni(
        self: Box<Self>,
    ) -> maybe_send::BoxedFuture<
        'static,
        Result<Box<dyn SendStream + 'static>, maybe_send_sync::BoxedError<'static>>,
    >;
}

impl<X> OpeningUni for X
where
    X: xwt_core::session::stream::OpeningUni,
    X: 'static,
{
    fn wait_uni(
        self: Box<Self>,
    ) -> maybe_send::BoxedFuture<
        'static,
        Result<Box<dyn SendStream + 'static>, maybe_send_sync::BoxedError<'static>>,
    > {
        Box::pin(async move {
            <X as xwt_core::session::stream::OpeningUni>::wait_uni(*self)
                .await
                .map(|val| Box::new(val) as _)
                .map_err(|error| Box::new(error) as _)
        })
    }
}

#[dyn_safe::dyn_safe(true)]
pub trait OpenUni: maybe::Send + maybe::Sync {
    fn open_uni(
        &self,
    ) -> maybe_send::BoxedFuture<
        '_,
        Result<Box<dyn OpeningUni + 'static>, maybe_send_sync::BoxedError<'static>>,
    >;
}

impl<X> OpenUni for X
where
    X: xwt_core::session::stream::OpenUni,
    X: maybe::Sync,
    X: 'static,
{
    fn open_uni(
        &self,
    ) -> maybe_send::BoxedFuture<
        '_,
        Result<Box<dyn OpeningUni + 'static>, maybe_send_sync::BoxedError<'static>>,
    > {
        Box::pin(async move {
            <X as xwt_core::session::stream::OpenUni>::open_uni(self)
                .await
                .map(|val| Box::new(val) as _)
                .map_err(|error| Box::new(error) as _)
        })
    }
}

#[dyn_safe::dyn_safe(true)]
pub trait AcceptUni: maybe::Send {
    fn accept_uni(
        &self,
    ) -> maybe_send::BoxedFuture<
        '_,
        Result<Box<dyn RecvStream + 'static>, maybe_send_sync::BoxedError<'static>>,
    >;
}

impl<X> AcceptUni for X
where
    X: xwt_core::session::stream::AcceptUni,
    X: maybe::Sync,
    X: 'static,
{
    fn accept_uni(
        &self,
    ) -> maybe_send::BoxedFuture<
        '_,
        Result<Box<dyn RecvStream + 'static>, maybe_send_sync::BoxedError<'static>>,
    > {
        Box::pin(async move {
            <X as xwt_core::session::stream::AcceptUni>::accept_uni(self)
                .await
                .map(|val| Box::new(val) as _)
                .map_err(|error| Box::new(error) as _)
        })
    }
}
