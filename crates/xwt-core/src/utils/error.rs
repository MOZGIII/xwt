//! An adapter for the error type.
//!
//! Experimental, and likely temporary.
//!
//! Required to specify the associated error type at trait requirements in
//! the `no_std` context, while the `Error` trait it still not stablilized
//! to be accessible from the [`core`].

#[cfg(feature = "std")]
pub use std::error::Error;

#[cfg(all(not(feature = "std"), feature = "error-in-core"))]
pub use core::error::Error;

#[cfg(all(not(feature = "std"), not(feature = "error-in-core")))]
/// An xwt error.
pub trait Error: core::fmt::Debug + core::fmt::Display {}

#[cfg(all(not(feature = "std"), not(feature = "error-in-core")))]
impl<T: core::fmt::Debug + core::fmt::Display> Error for T {}
