//! Operations on the WebTransport streams.

use core::num::NonZeroUsize;

use alloc::boxed::Box;
use xwt_core::utils::maybe;

use crate::utils::traits::{maybe_send, maybe_send_sync};

#[dyn_safe::dyn_safe(true)]
pub trait AsErrorCode: maybe::Send {
    fn as_error_code(&self) -> Option<Result<u32, Box<dyn core::any::Any + 'static>>>;
}

impl<X> AsErrorCode for X
where
    X: xwt_core::stream::AsErrorCode<ErrorCode: TryInto<u32> + 'static>,
{
    fn as_error_code(&self) -> Option<Result<u32, Box<dyn core::any::Any + 'static>>> {
        let error_code = <X as xwt_core::stream::AsErrorCode>::as_error_code(self)?;
        let error_code = match error_code.try_into() {
            Ok(val) => val,
            Err(error) => {
                return Some(Err(Box::new(error)));
            }
        };
        Some(Ok(error_code))
    }
}

#[dyn_safe::dyn_safe(true)]
pub trait OpError: maybe_send_sync::Error + AsErrorCode {}

impl<X> OpError for X where X: maybe_send_sync::Error + AsErrorCode {}

#[dyn_safe::dyn_safe(true)]
pub trait Read: maybe::Send {
    fn read<'a>(
        &'a mut self,
        buf: &'a mut [u8],
    ) -> maybe_send::BoxedFuture<'a, Result<NonZeroUsize, Box<dyn OpError + 'static>>>;
}

impl<X> Read for X
where
    X: xwt_core::stream::Read,
{
    fn read<'a>(
        &'a mut self,
        buf: &'a mut [u8],
    ) -> maybe_send::BoxedFuture<'a, Result<NonZeroUsize, Box<dyn OpError + 'static>>> {
        Box::pin(async move {
            <X as xwt_core::stream::Read>::read(self, buf)
                .await
                .map_err(|error| Box::new(error) as _)
        })
    }
}

#[dyn_safe::dyn_safe(true)]
pub trait Write: maybe::Send {
    fn write<'a>(
        &'a mut self,
        buf: &'a [u8],
    ) -> maybe_send::BoxedFuture<'a, Result<NonZeroUsize, Box<dyn OpError + 'static>>>;
}

impl<X> Write for X
where
    X: xwt_core::stream::Write,
{
    fn write<'a>(
        &'a mut self,
        buf: &'a [u8],
    ) -> maybe_send::BoxedFuture<'a, Result<NonZeroUsize, Box<dyn OpError + 'static>>> {
        Box::pin(async move {
            <X as xwt_core::stream::Write>::write(self, buf)
                .await
                .map_err(|error| Box::new(error) as _)
        })
    }
}

#[dyn_safe::dyn_safe(true)]
pub trait ReadAbort: maybe::Send {
    fn abort(
        self: Box<Self>,
        error_code: u32,
    ) -> maybe_send::BoxedFuture<'static, Result<(), maybe_send_sync::BoxedError<'static>>>;
}

impl<X> ReadAbort for X
where
    X: xwt_core::stream::ReadAbort,
    X: 'static,
{
    fn abort(
        self: Box<Self>,
        error_code: u32,
    ) -> maybe_send::BoxedFuture<'static, Result<(), maybe_send_sync::BoxedError<'static>>> {
        let error_code = error_code.into();
        Box::pin(async move {
            <X as xwt_core::stream::ReadAbort>::abort(*self, error_code)
                .await
                .map_err(|error| Box::new(error) as _)
        })
    }
}

#[dyn_safe::dyn_safe(true)]
pub trait WriteAbort: maybe::Send {
    fn abort(
        self: Box<Self>,
        error_code: u32,
    ) -> maybe_send::BoxedFuture<'static, Result<(), maybe_send_sync::BoxedError<'static>>>;
}

impl<X> WriteAbort for X
where
    X: xwt_core::stream::WriteAbort,
    X: 'static,
{
    fn abort(
        self: Box<Self>,
        error_code: u32,
    ) -> maybe_send::BoxedFuture<'static, Result<(), maybe_send_sync::BoxedError<'static>>> {
        let error_code = error_code.into();
        Box::pin(async move {
            <X as xwt_core::stream::WriteAbort>::abort(*self, error_code)
                .await
                .map_err(|error| Box::new(error) as _)
        })
    }
}

#[dyn_safe::dyn_safe(true)]
pub trait WriteAborted: maybe::Send {
    fn aborted(
        self: Box<Self>,
    ) -> maybe_send::BoxedFuture<
        'static,
        Result<
            Result<u32, Box<dyn core::any::Any + 'static>>,
            maybe_send_sync::BoxedError<'static>,
        >,
    >;
}

impl<X> WriteAborted for X
where
    X: xwt_core::stream::WriteAborted,
    X: 'static,
{
    fn aborted(
        self: Box<Self>,
    ) -> maybe_send::BoxedFuture<
        'static,
        Result<
            Result<u32, Box<dyn core::any::Any + 'static>>,
            maybe_send_sync::BoxedError<'static>,
        >,
    > {
        Box::pin(async move {
            <X as xwt_core::stream::WriteAborted>::aborted(*self)
                .await
                .map_err(|error| Box::new(error) as _)
                .map(|error_code| error_code.try_into().map_err(|error| Box::new(error) as _))
        })
    }
}

#[dyn_safe::dyn_safe(true)]
pub trait Finish: maybe::Send {
    fn finish(
        self: Box<Self>,
    ) -> maybe_send::BoxedFuture<'static, Result<(), maybe_send_sync::BoxedError<'static>>>;
}

impl<X> Finish for X
where
    X: xwt_core::stream::Finish,
    X: 'static,
{
    fn finish(
        self: Box<Self>,
    ) -> maybe_send::BoxedFuture<'static, Result<(), maybe_send_sync::BoxedError<'static>>> {
        Box::pin(async move {
            <X as xwt_core::stream::Finish>::finish(*self)
                .await
                .map_err(|error| Box::new(error) as _)
        })
    }
}

#[dyn_safe::dyn_safe(true)]
pub trait ReadChunkU8: maybe::Send {
    fn read_chunk(
        &mut self,
        max_length: usize,
        ordered: bool,
    ) -> maybe_send::BoxedFuture<
        '_,
        Result<
            Option<xwt_core::stream::Chunk<alloc::vec::Vec<u8>>>,
            maybe_send_sync::BoxedError<'static>,
        >,
    >;
}

#[cfg(feature = "alloc")]
impl<X> ReadChunkU8 for X
where
    X: xwt_core::stream::ReadChunk<xwt_core::stream::chunk::U8>,
    X: 'static,
{
    fn read_chunk(
        &mut self,
        max_length: usize,
        ordered: bool,
    ) -> maybe_send::BoxedFuture<
        '_,
        Result<
            Option<xwt_core::stream::Chunk<alloc::vec::Vec<u8>>>,
            maybe_send_sync::BoxedError<'static>,
        >,
    > {
        Box::pin(async move {
            <X as xwt_core::stream::ReadChunk<xwt_core::stream::chunk::U8>>::read_chunk(
                self, max_length, ordered,
            )
            .await
            .map_err(|error| Box::new(error) as _)
        })
    }
}

#[dyn_safe::dyn_safe(true)]
pub trait WriteChunkU8: maybe::Send {
    fn write_chunk<'a>(
        &'a mut self,
        buf: &'a [u8],
    ) -> maybe_send::BoxedFuture<'a, Result<(), maybe_send_sync::BoxedError<'static>>>;
}

#[cfg(feature = "alloc")]
impl<X> WriteChunkU8 for X
where
    X: xwt_core::stream::WriteChunk<xwt_core::stream::chunk::U8>,
    X: 'static,
{
    fn write_chunk<'a>(
        &'a mut self,
        buf: &'a [u8],
    ) -> maybe_send::BoxedFuture<'a, Result<(), maybe_send_sync::BoxedError<'static>>> {
        Box::pin(async move {
            <X as xwt_core::stream::WriteChunk<xwt_core::stream::chunk::U8>>::write_chunk(self, buf)
                .await
                .map_err(|error| Box::new(error) as _)
        })
    }
}
