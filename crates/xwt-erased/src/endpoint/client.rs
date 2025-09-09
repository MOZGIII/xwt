#![allow(missing_docs)]

use alloc::boxed::Box;

use crate::Error;

pub struct Client {
    endpoint: Box<dyn xwt_dyn::endpoint::Connect>,
}

impl Client {
    pub fn new(val: impl xwt_dyn::endpoint::Connect + 'static) -> Self {
        Self {
            endpoint: Box::new(val) as _,
        }
    }

    pub const fn from_endpoint(endpoint: Box<dyn xwt_dyn::endpoint::Connect>) -> Self {
        Self { endpoint }
    }

    pub async fn connect(&self, url: &str) -> Result<Connecting, Error> {
        self.endpoint
            .connect(url)
            .await
            .map(Connecting)
            .map_err(Error::from_inner)
    }
}

pub struct Connecting(Box<dyn xwt_dyn::endpoint::connect::Connecting + 'static>);

impl Connecting {
    pub async fn wait_connect(self) -> Result<crate::Session, Error> {
        self.0
            .wait_connect()
            .await
            .map(crate::Session::from_inner)
            .map_err(Error::from_inner)
    }
}
