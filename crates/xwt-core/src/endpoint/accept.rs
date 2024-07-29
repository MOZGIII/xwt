#![allow(missing_docs)]

use core::future::Future;

use crate::utils::{maybe, Error};

pub trait Accept: Sized + maybe::Send {
    type Accepting: Accepting;
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn accept(
        &self,
    ) -> impl Future<Output = Result<Option<Self::Accepting>, Self::Error>> + maybe::Send;
}

pub trait Accepting: maybe::Send {
    type Request: Request;
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn wait_accept(self) -> impl Future<Output = Result<Self::Request, Self::Error>> + maybe::Send;
}

pub trait Request: maybe::Send {
    type Session: maybe::Send;
    type OkError: Error + maybe::Send + maybe::Sync + 'static;
    type CloseError: Error + maybe::Send + maybe::Sync + 'static;

    fn ok(self) -> impl Future<Output = Result<Self::Session, Self::OkError>> + maybe::Send;

    fn close(self, status: u16)
        -> impl Future<Output = Result<(), Self::CloseError>> + maybe::Send;
}
