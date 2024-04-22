//! Various conversions into/from other types.

use crate::types::*;

/// Implement all traits for a given type.
macro_rules! implement {
    (
        $($t:ty,)*
    ) => {
        $(
            impl<T> $t {
                /// Create an instance from a value of the underlying type `T`
                /// with a `const` context.
                pub const fn from_const(value: T) -> Self {
                    Self(value)
                }

                /// Take the underlying value out.
                pub fn into_inner(self) -> T {
                    self.0
                }

                /// Take a ref to the underlying value.
                pub const fn inner_ref(&self) -> &T {
                    &self.0
                }

                /// Take a mut ref to the underlying value.
                pub fn inner_mut(&mut self) -> &mut T {
                    &mut self.0
                }
            }
        )*
    };
}

crate::for_all_material_types!(implement);
