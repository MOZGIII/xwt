//! Test server routes.

pub mod close;
pub mod echo;
pub mod echo_open_bi;

/// All test server routes.
pub type Routes = (
    echo::Route,
    echo_open_bi::Route,
    close::CloseUni,
    close::CloseUniError,
    close::CloseBiRecv,
    close::CloseBiRecvError,
    close::CloseBiSend,
    close::CloseBiSendError,
);
