//! [`From`].

use crate::types::*;

/// Implement all traits for a given type.
macro_rules! implement {
    (
        $($t:ty,)*
    ) => {
        $(
            impl<T> From<T> for $t {
                fn from(value: T) -> Self {
                    Self(value)
                }
            }
        )*
    };
}

crate::for_all_material_types!(implement);
