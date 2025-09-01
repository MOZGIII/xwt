use crate::*;

impl std::fmt::Debug for Connect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Connect::Connect(inner) => f.debug_tuple("Connect::Connect").field(inner).finish(),
            Connect::Connecting(inner) => {
                f.debug_tuple("Connect::Connecting").field(inner).finish()
            }
        }
    }
}

impl std::fmt::Debug for Accept {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Accept::Accept(inner) => f.debug_tuple("Accept::Accept").field(inner).finish(),
        }
    }
}

impl std::fmt::Debug for Accepting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Accepting::Accepting(inner) => f.debug_tuple("Accept::Accepting").field(inner).finish(),
            Accepting::RequestOk(inner) => f.debug_tuple("Accept::RequestOk").field(inner).finish(),
            Accepting::RequestClose(inner) => {
                f.debug_tuple("Accept::RequestClose").field(inner).finish()
            }
        }
    }
}

impl std::fmt::Debug for OpenBi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenBi::Open(inner) => f.debug_tuple("OpenBi::Open").field(inner).finish(),
            OpenBi::Opening(inner) => f.debug_tuple("OpenBi::Opening").field(inner).finish(),
        }
    }
}

impl std::fmt::Debug for OpenUni {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenUni::Open(inner) => f.debug_tuple("OpenUni::Open").field(inner).finish(),
            OpenUni::Opening(inner) => f.debug_tuple("OpenUni::Opening").field(inner).finish(),
        }
    }
}
