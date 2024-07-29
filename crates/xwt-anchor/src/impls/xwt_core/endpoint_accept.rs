//! Implementations related to endpoint accept operations.

use xwt_core::utils::maybe;

use crate::types::*;

impl<T> xwt_core::endpoint::Accept for Endpoint<T>
where
    T: xwt_core::endpoint::Accept + maybe::Send + maybe::Sync,
{
    type Accepting = Accepting<T::Accepting>;
    type Error = T::Error;

    async fn accept(&self) -> Result<Option<Self::Accepting>, Self::Error> {
        T::accept(&self.0).await.map(|e| e.map(Accepting))
    }
}

impl<T> xwt_core::endpoint::accept::Accepting for Accepting<T>
where
    T: xwt_core::endpoint::accept::Accepting,
{
    type Request = Request<T::Request>;
    type Error = T::Error;

    async fn wait_accept(self) -> Result<Self::Request, Self::Error> {
        T::wait_accept(self.0).await.map(Request)
    }
}

impl<T> xwt_core::endpoint::accept::Request for Request<T>
where
    T: xwt_core::endpoint::accept::Request,
{
    type Session = Session<T::Session>;
    type OkError = T::OkError;
    type CloseError = T::CloseError;

    async fn ok(self) -> Result<Self::Session, Self::OkError> {
        T::ok(self.0).await.map(Session)
    }

    async fn close(self, status: u16) -> Result<(), Self::CloseError> {
        T::close(self.0, status).await
    }
}
