#![allow(missing_docs)]

use super::datagram;

pub type SendErrorFor<T> = <T as datagram::Send>::Error;

pub type ReceiveErrorFor<T> = <T as datagram::Receive>::Error;

pub type ReceiveDatagramFor<T> = <T as datagram::Receive>::Datagram;

pub type ReceiveIntoErrorFor<T> = <T as datagram::ReceiveInto>::Error;
