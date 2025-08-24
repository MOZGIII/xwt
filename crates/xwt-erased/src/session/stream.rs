#![allow(missing_docs)]

use alloc::boxed::Box;

use crate::stream::{RecvStream, SendStream};

use super::Session;

pub struct OpeningBi(Box<dyn xwt_dyn::session::stream::OpeningBi>);

pub struct OpeningUni(Box<dyn xwt_dyn::session::stream::OpeningUni>);

impl Session {
    pub async fn open_bi(&self) -> Result<OpeningBi, crate::Error> {
        self.0
            .open_bi()
            .await
            .map(OpeningBi)
            .map_err(crate::Error::from_inner)
    }

    pub async fn open_uni(&self) -> Result<OpeningUni, crate::Error> {
        self.0
            .open_uni()
            .await
            .map(OpeningUni)
            .map_err(crate::Error::from_inner)
    }

    pub async fn accept_bi(&self) -> Result<(SendStream, RecvStream), crate::Error> {
        self.0
            .accept_bi()
            .await
            .map(|(tx, rx)| (SendStream::from_inner(tx), RecvStream::from_inner(rx)))
            .map_err(crate::Error::from_inner)
    }

    pub async fn accept_uni(&self) -> Result<RecvStream, crate::Error> {
        self.0
            .accept_uni()
            .await
            .map(RecvStream::from_inner)
            .map_err(crate::Error::from_inner)
    }
}

impl OpeningBi {
    pub async fn wait_bi(self) -> Result<(SendStream, RecvStream), crate::Error> {
        self.0
            .wait_bi()
            .await
            .map(|(tx, rx)| (SendStream::from_inner(tx), RecvStream::from_inner(rx)))
            .map_err(crate::Error::from_inner)
    }
}

impl OpeningUni {
    pub async fn wait_uni(self) -> Result<SendStream, crate::Error> {
        self.0
            .wait_uni()
            .await
            .map(SendStream::from_inner)
            .map_err(crate::Error::from_inner)
    }
}
