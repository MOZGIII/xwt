use alloc::boxed::Box;

pub mod datagram;
pub mod stream;

pub struct Session(Box<dyn xwt_dyn::base::Session>);

impl Session {
    pub fn into_inner(self) -> Box<dyn xwt_dyn::base::Session> {
        self.0
    }

    pub const fn from_inner(inner: Box<dyn xwt_dyn::base::Session>) -> Self {
        Self(inner)
    }
}
