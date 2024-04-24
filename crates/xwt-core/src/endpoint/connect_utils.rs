#![allow(missing_docs)]

use super::connect;

pub type ConnectSessionFor<T> = ConnectingSessionFor<<T as connect::Connect>::Connecting>;

pub type ConnectingSessionFor<T> = <T as connect::Connecting>::Session;
