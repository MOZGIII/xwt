#![allow(missing_docs)]

use core::future::Future;

use crate::utils::{maybe, Error};

pub trait Connect: maybe::Send {
    type Connecting: Connecting + 'static;
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn connect(
        &self,
        url: &str,
    ) -> impl Future<Output = Result<Self::Connecting, Self::Error>> + maybe::Send;
}

pub trait Connecting: maybe::Send {
    type Session: maybe::Send;
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn wait_connect(self)
        -> impl Future<Output = Result<Self::Session, Self::Error>> + maybe::Send;
}
