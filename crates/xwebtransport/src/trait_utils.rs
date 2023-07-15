use crate::traits;

pub type EndpointConnectConnectionFor<T> =
    <<T as traits::EndpointConnect>::Connecting as traits::Connecting>::Connection;

pub type EndpointAcceptConnectionFor<T> =
    <<T as traits::EndpointAccept>::Connecting as traits::Connecting>::Connection;
