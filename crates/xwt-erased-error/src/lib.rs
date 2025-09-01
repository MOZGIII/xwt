//! A suite of reusable error types that naturally emerge from the `xwt` API.
//! Use when you don't want/need your own more precise types.

#![allow(missing_docs, clippy::missing_docs_in_private_items)]

mod impls;

use xwt_erased::Error;

pub enum Connect {
    Connect(Error),
    Connecting(Error),
}

pub enum Accept {
    Accept(Error),
}

pub enum Accepting {
    Accepting(Error),
    RequestOk(Error),
    RequestClose(Error),
}

pub enum OpenBi {
    Open(Error),
    Opening(Error),
}

pub enum OpenUni {
    Open(Error),
    Opening(Error),
}
