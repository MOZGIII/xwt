use crate::*;

impl std::fmt::Display for Connect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Connect::Connect(inner) => write!(f, "connect: {inner}"),
            Connect::Connecting(inner) => write!(f, "connecting: {inner}"),
        }
    }
}

impl std::fmt::Display for Accept {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Accept::Accept(inner) => write!(f, "accept: {inner}"),
        }
    }
}

impl std::fmt::Display for Accepting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Accepting::Accepting(inner) => write!(f, "accepting: {inner}"),
            Accepting::RequestOk(inner) => write!(f, "oking request: {inner}"),
            Accepting::RequestClose(inner) => write!(f, "closing request: {inner}"),
        }
    }
}

impl std::fmt::Display for OpenBi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenBi::Open(inner) => write!(f, "open: {inner}"),
            OpenBi::Opening(inner) => write!(f, "opening: {inner}"),
        }
    }
}

impl std::fmt::Display for OpenUni {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenUni::Open(inner) => write!(f, "open: {inner}"),
            OpenUni::Opening(inner) => write!(f, "opening: {inner}"),
        }
    }
}
