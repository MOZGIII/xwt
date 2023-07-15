use crate::Streams;

pub struct Connecting<T>(pub T);

impl<T> crate::traits::Connecting for Connecting<T> {
    type Connection = T;
    type Error = std::convert::Infallible;

    async fn wait(self) -> Result<Self::Connection, Self::Error> {
        Ok(self.0)
    }
}

pub struct OpeningUniStream<T: Streams>(pub T::SendStream);

impl<T: Streams> crate::traits::OpeningUniStream for OpeningUniStream<T> {
    type Streams = T;
    type Error = std::convert::Infallible;

    async fn wait(self) -> Result<<T as Streams>::SendStream, Self::Error> {
        Ok(self.0)
    }
}

pub struct OpeningBiStream<T: Streams>(pub (T::SendStream, T::RecvStream));

impl<T: Streams> crate::traits::OpeningBiStream for OpeningBiStream<T> {
    type Streams = T;
    type Error = std::convert::Infallible;

    async fn wait(
        self,
    ) -> Result<(<T as Streams>::SendStream, <T as Streams>::RecvStream), Self::Error> {
        Ok(self.0)
    }
}
