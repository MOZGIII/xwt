use crate::session;

#[dyn_safe::dyn_safe(true)]
pub trait Session: session::base::StreamOps + session::base::DatagramOps {}

impl<T> Session for T where T: session::base::StreamOps + session::base::DatagramOps {}
