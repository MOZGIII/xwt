//! The anchor types for [`xwt_core`].
//!
//! Intended to be useful in overcoming the restriction on the implementations
//! for foreign types.
//!
//! That is - if you want to implement something for [`xwt_core`] but can't -
//! implement it concretely for an `xwt_anchor` type and switch to using
//! `xwt_anchor`-wrapped types instead of raw `xwt_core` traits where
//! applicable.
//!
//! This implementation is experimental, and we'll see on whether we'll keep
//! maintaining it.

#![no_std]

mod impls;
mod macros;
mod types;

pub use types::*;

pub use xwt_core as core;

/// Commonly used types and traits.
///
/// # Usage
///
/// ```
/// use xwt_anchor::prelude::*;
/// ```
///
/// Note that `use xwt_anchor::prelude::*;` is different from
/// `use xwt_anchor::prelude::{a, c, c};` in that exported but unnamable items
/// can't be imported by enumrating them, while `::*` *will* load them
/// into scope.
pub mod prelude {
    pub use xwt_core::prelude::*;
}
