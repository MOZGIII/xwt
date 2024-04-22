//! [`AsRef`] and [`AsMut`].

use crate::types::*;

/// Implement all traits for a given type.
macro_rules! implement {
    (
        $($t:ty,)*
    ) => {
        $(
            impl<T> AsRef<T> for $t {
                fn as_ref(&self) -> &T {
                    &self.0
                }
            }

            impl<T> AsMut<T> for $t {
                fn as_mut(&mut self) -> &mut T {
                    &mut self.0
                }
            }
        )*
    };
}

crate::for_all_material_types!(implement);
