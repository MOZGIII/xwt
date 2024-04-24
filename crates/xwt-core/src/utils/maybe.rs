//! Marker traits for enabling [`Send`] and [`Sync`] requirements based on the
//! target family.
//!
//! Normally we want to require real [`core::marker::Send`] and
//! [`core::marker::Sync`] for most of the types, however, the Web API bindings
//! can not support those, and in the Web setting, the spawning of async tasks
//! does not currently require those either.
//! So we turn off the [`core::marker::Send`] and [`core::marker::Sync`]
//! requirements for the Web targets using these [`Send`] and [`Sync`] traits.

/// Maybe a [`core::marker::Send`].
///
/// For this target - a real [`core::marker::Send`] requirement.
#[cfg(not(target_family = "wasm"))]
pub trait Send: core::marker::Send {}

#[cfg(not(target_family = "wasm"))]
impl<T> self::Send for T where T: core::marker::Send {}

/// Maybe a [`core::marker::Send`].
///
/// For this target - NO [`core::marker::Send`] requirement.
#[cfg(target_family = "wasm")]
pub trait Send {}

#[cfg(target_family = "wasm")]
impl<T> self::Send for T {}

/// Maybe a [`core::marker::Sync`].
///
/// For this target - a real [`core::marker::Sync`] requirement.
#[cfg(not(target_family = "wasm"))]
pub trait Sync: core::marker::Sync {}

#[cfg(not(target_family = "wasm"))]
impl<T> self::Sync for T where T: core::marker::Sync {}

/// Maybe a [`core::marker::Sync`].
///
/// For this target - NO [`core::marker::Sync`] requirement.
#[cfg(target_family = "wasm")]
pub trait Sync {}

#[cfg(target_family = "wasm")]
impl<T> self::Sync for T {}
