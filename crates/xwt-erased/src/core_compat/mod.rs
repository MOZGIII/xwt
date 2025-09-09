mod endpoint {
    mod client;
    mod server;
}

mod session {
    mod datagram;
    mod stream;
}

mod stream {
    mod error;
    mod recv;
    mod send;
}

static_assertions::assert_impl_all!(crate::Client: xwt_core::endpoint::Connect);
static_assertions::assert_impl_all!(crate::Server: xwt_core::endpoint::Accept);
static_assertions::assert_impl_all!(crate::Session: xwt_core::base::Session);
