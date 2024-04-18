use core::future::Future;

use crate::utils::{maybe, Error};

pub trait Receive: maybe::Send {
    type Datagram: maybe::Send + AsRef<[u8]>;
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn receive_datagram(
        &self,
    ) -> impl Future<Output = Result<Self::Datagram, Self::Error>> + maybe::Send;
}

pub trait ReceiveInto: maybe::Send {
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn receive_datagram_into(
        &self,
        buf: &mut [u8],
    ) -> impl Future<Output = Result<usize, Self::Error>> + maybe::Send;
}

pub trait Send: maybe::Send {
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn send_datagram<D>(
        &self,
        payload: D,
    ) -> impl Future<Output = Result<(), Self::Error>> + maybe::Send
    where
        D: maybe::Send + AsRef<[u8]>;
}

pub trait Datagrams: Send + Receive + ReceiveInto {}

impl<T> Datagrams for T where T: Send + Receive + ReceiveInto {}
