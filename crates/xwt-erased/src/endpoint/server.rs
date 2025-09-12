#![allow(missing_docs)]

use alloc::boxed::Box;

use crate::Error;

pub struct Server {
    endpoint: Box<dyn xwt_dyn::endpoint::Accept>,
}

impl Server {
    pub fn new(val: impl xwt_dyn::endpoint::Accept + 'static) -> Self {
        Self {
            endpoint: Box::new(val) as _,
        }
    }

    pub const fn from_endpoint(endpoint: Box<dyn xwt_dyn::endpoint::Accept>) -> Self {
        Self { endpoint }
    }

    pub async fn accept(&self) -> Result<Option<Accepting>, Error> {
        crate::trace_call!();
        self.endpoint
            .accept()
            .await
            .map(|val| val.map(Accepting))
            .map_err(Error::from_inner)
    }
}

pub struct Accepting(Box<dyn xwt_dyn::endpoint::accept::Accepting + 'static>);

impl Accepting {
    pub async fn wait_accept(self) -> Result<Request, Error> {
        crate::trace_call!();
        self.0
            .wait_accept()
            .await
            .map(Request)
            .map_err(Error::from_inner)
    }
}

pub struct Request(Box<dyn xwt_dyn::endpoint::accept::Request>);

impl Request {
    pub async fn ok(self) -> Result<crate::Session, Error> {
        crate::trace_call!();
        self.0
            .ok()
            .await
            .map(crate::Session::from_inner)
            .map_err(Error::from_inner)
    }

    pub async fn close(self, status: u16) -> Result<(), Error> {
        crate::trace_call!();
        self.0.close(status).await.map_err(Error::from_inner)
    }
}
