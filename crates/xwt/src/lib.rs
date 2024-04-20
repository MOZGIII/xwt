//! The ╳-platform WebTransport implementation.
//!
//! Cross-platform interface for working with WebTransport.

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
/// use xwt::prelude::*;
/// ```
///
/// Note that `use xwt::prelude::*;` is different
/// from `use xwt::prelude::{a, c, c};` in that unnamable yet exported items
/// can't be imported by enumrating them, while `::*` *will* load them into
/// scope.
pub mod prelude {
    pub use xwt_core::prelude::*;
}
