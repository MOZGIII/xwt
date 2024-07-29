//! Test server routes.

pub mod echo;
pub mod echo_open_bi;

/// All test server routes.
pub type Routes = (echo::Route, echo_open_bi::Route);
