//! Derefs.

use crate::types::*;

/// Implement all traits for a given type.
macro_rules! implement {
    (
        $($t:ty,)*
    ) => {
        $(
            impl<T> core::ops::Deref for $t {
                type Target = T;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl<T> core::ops::DerefMut for $t {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        )*
    };
}

crate::for_all_material_types!(implement);
