#![allow(missing_docs)]

use super::accept;

pub type AcceptSessionFor<T> = RequestSessionFor<AcceptRequestFor<T>>;

pub type AcceptingRequestFor<T> = <T as accept::Accepting>::Request;

pub type AcceptRequestFor<T> = AcceptingRequestFor<<T as accept::Accept>::Accepting>;

pub type RequestSessionFor<T> = <T as accept::Request>::Session;

pub type AcceptingSessionFor<T> = RequestSessionFor<<T as accept::Accepting>::Request>;
