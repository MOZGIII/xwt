#[cfg(not(target_family = "wasm"))]
pub trait Send: core::marker::Send {}

#[cfg(not(target_family = "wasm"))]
impl<T> self::Send for T where T: core::marker::Send {}

#[cfg(target_family = "wasm")]
pub trait Send {}

#[cfg(target_family = "wasm")]
impl<T> self::Send for T {}

#[cfg(not(target_family = "wasm"))]
pub trait Sync: core::marker::Sync {}

#[cfg(not(target_family = "wasm"))]
impl<T> self::Sync for T where T: core::marker::Sync {}

#[cfg(target_family = "wasm")]
pub trait Sync {}

#[cfg(target_family = "wasm")]
impl<T> self::Sync for T {}
