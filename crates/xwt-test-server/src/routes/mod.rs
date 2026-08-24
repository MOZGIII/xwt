//! Test server routes.

pub mod abort;
pub mod close;
pub mod echo;
pub mod echo_open_bi;
pub mod echo_uni;

/// All test server routes.
pub type Routes = (
    echo::Route,
    echo_open_bi::Route,
    echo_uni::Route,
    close::CloseUni,
    close::CloseUniError,
    close::CloseBiRecv,
    close::CloseBiRecvError,
    close::CloseBiSend,
    close::CloseBiSendError,
    abort::AbortBiSend,
);
