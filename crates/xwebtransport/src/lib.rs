#![feature(async_fn_in_trait)]
#![feature(associated_type_bounds)]

pub mod trait_utils;
pub mod traits;

pub mod r#impl {
    #[cfg(all(feature = "web-sys", target_family = "wasm"))]
    pub mod web_sys;
    #[cfg(all(feature = "wtransport", not(target_family = "wasm")))]
    pub mod wtransport;
}

#[cfg(all(feature = "web-sys", target_family = "wasm"))]
pub use r#impl::web_sys as default;
#[cfg(all(feature = "web-sys", not(target_family = "wasm")))]
pub use r#impl::wtransport as default;

pub mod utils {
    pub mod dummy;
    pub mod maybe;
}

pub use traits::*;

pub mod prelude {
    pub use crate::traits::{
        AcceptBiStream as _, AcceptUniStream as _, Connecting as _, Connection as _,
        EndpointAccept as _, EndpointConnect as _, OpenBiStream as _, OpenUniStream as _,
        OpeningBiStream as _, OpeningUniStream as _,
    };

    pub use crate::trait_utils::*;
}
