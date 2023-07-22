pub mod trait_utils;
pub mod traits;

pub mod utils {
    pub mod dummy;
    pub mod maybe;
}

pub use async_trait::async_trait;

pub use traits::*;

pub mod prelude {
    pub use crate::traits::{
        AcceptBiStream as _, AcceptUniStream as _, Accepting as _, Connecting as _,
        Connection as _, EndpointAccept as _, EndpointConnect as _, OpenBiStream as _,
        OpenUniStream as _, OpeningBiStream as _, OpeningUniStream as _, Request as _,
    };

    pub use crate::trait_utils::*;
}
