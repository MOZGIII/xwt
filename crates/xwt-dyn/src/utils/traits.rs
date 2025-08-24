//! Utility traits.

use xwt_core::utils::maybe;

pub mod maybe_send {
    use super::*;

    pub trait Future<T>: core::future::Future<Output = T> + maybe::Send {}
    pub trait Error: core::error::Error + maybe::Send {}
    pub trait Any: core::any::Any + maybe::Send {}

    impl<X, T> Future<T> for X where X: core::future::Future<Output = T> + maybe::Send {}
    impl<X> Error for X where X: core::error::Error + maybe::Send {}
    impl<X> Any for X where X: core::any::Any + maybe::Send {}

    crate::boxed!();
}

pub mod maybe_send_sync {
    use super::*;

    pub trait Future<T>: core::future::Future<Output = T> + maybe::Send + maybe::Sync {}
    pub trait Error: core::error::Error + maybe::Send + maybe::Sync {}
    pub trait Any: core::any::Any + maybe::Send + maybe::Sync {}

    impl<X, T> Future<T> for X where X: core::future::Future<Output = T> + maybe::Send + maybe::Sync {}
    impl<X> Error for X where X: core::error::Error + maybe::Send + maybe::Sync {}
    impl<X> Any for X where X: core::any::Any + maybe::Send + maybe::Sync {}

    crate::boxed!();
}

#[macro_export]
macro_rules! boxed {
    () => {
        mod boxed {
            use alloc::boxed::Box;

            pub type BoxedFuture<'a, T> = core::pin::Pin<Box<dyn super::Future<T> + 'a>>;
            pub type BoxedError<'a> = Box<dyn super::Error + 'a>;
            pub type BoxedAny<'a> = Box<dyn super::Any + 'a>;
        }
        pub use boxed::*;
    };
}
