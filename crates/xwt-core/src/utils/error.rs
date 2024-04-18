#[cfg(feature = "std")]
pub use std::error::Error;

#[cfg(all(not(feature = "std"), feature = "error-in-core"))]
pub use core::error::Error;

#[cfg(all(not(feature = "std"), not(feature = "error-in-core")))]
pub trait Error: core::fmt::Debug + core::fmt::Display {}

#[cfg(all(not(feature = "std"), not(feature = "error-in-core")))]
impl<T: core::fmt::Debug + core::fmt::Display> Error for T {}
