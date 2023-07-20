use crate::traits;

pub type EndpointConnectConnectionFor<T> =
    <<T as traits::EndpointConnect>::Connecting as traits::Connecting>::Connection;

pub type EndpointAcceptConnectionFor<T> =
    <<T as traits::EndpointAccept>::Connecting as traits::Connecting>::Connection;

pub type BiStreamOpeningErrorFor<T> =
    <<T as traits::OpenBiStream>::Opening as traits::OpeningBiStream>::Error;

pub type UniStreamOpeningErrorFor<T> =
    <<T as traits::OpenUniStream>::Opening as traits::OpeningUniStream>::Error;

pub type BiStreamsFor<T> = traits::BiStreamsFor<
    <<T as traits::OpenBiStream>::Opening as traits::OpeningBiStream>::Streams,
>;

pub type SendUniStreamFor<T> =
    <<<T as traits::OpenUniStream>::Opening as traits::OpeningUniStream>::Streams as traits::Streams>::SendStream;

pub type RecvUniStreamFor<T> =
    <<<T as traits::OpenUniStream>::Opening as traits::OpeningUniStream>::Streams as traits::Streams>::RecvStream;
