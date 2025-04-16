//! Newtype definitions.

/// Create a newtype wrapper for a given type.
macro_rules! newtype {
    ($name:ident => $wrapped_type:path) => {
        #[doc = concat!("The [`", stringify!($wrapped_type), "`] newtype.")]
        pub struct $name(pub $wrapped_type);

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($name)).finish()
            }
        }

        impl From<$wrapped_type> for $name {
            fn from(value: $wrapped_type) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $wrapped_type {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
    ($name:ident < $($generics:tt),* > => $wrapped_type:path) => {
        #[doc = concat!("The [`", stringify!($wrapped_type), "`] newtype.")]
        pub struct $name<$($generics)*>(pub $wrapped_type);

        impl<$($generics)*> std::fmt::Debug for $name<$($generics)*> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($name)).finish()
            }
        }

        impl<$($generics)*> From<$wrapped_type> for $name<$($generics)*> {
            fn from(value: $wrapped_type) -> Self {
                Self(value)
            }
        }

        impl<$($generics)*> From<$name<$($generics)*>> for $wrapped_type {
            fn from(value: $name<$($generics)*>) -> Self {
                value.0
            }
        }

        impl<$($generics)*> std::ops::Deref for $name<$($generics)*> {
            type Target = $wrapped_type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<$($generics)*> std::ops::DerefMut for $name<$($generics)*> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

/// Create a newtype wrapper with pin projection for a given type.

/// Create a newtype wrapper for a given type.
macro_rules! newtype_pin {
    ($name:ident => $wrapped_type:path) => {
        #[pin_project::pin_project]
        #[doc = concat!("The [`", stringify!($wrapped_type), "`] newtype.")]
        pub struct $name(#[pin] pub $wrapped_type);

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($name)).finish()
            }
        }

        impl From<$wrapped_type> for $name {
            fn from(value: $wrapped_type) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $wrapped_type {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
    ($name:ident < $($generics:tt),* > => $wrapped_type:path) => {
        #[doc = concat!("The [`", stringify!($wrapped_type), "`] newtype.")]
        pub struct $name<$($generics)*>(pub $wrapped_type);

        impl<$($generics)*> std::fmt::Debug for $name<$($generics)*> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($name)).finish()
            }
        }

        impl<$($generics)*> From<$wrapped_type> for $name<$($generics)*> {
            fn from(value: $wrapped_type) -> Self {
                Self(value)
            }
        }

        impl<$($generics)*> From<$name<$($generics)*>> for $wrapped_type {
            fn from(value: $name<$($generics)*>) -> Self {
                value.0
            }
        }

        impl<$($generics)*> std::ops::Deref for $name<$($generics)*> {
            type Target = $wrapped_type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<$($generics)*> std::ops::DerefMut for $name<$($generics)*> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

newtype!(Endpoint<Side> => wtransport::Endpoint<Side>);
newtype!(Connection => wtransport::Connection);
newtype!(IncomingSession => wtransport::endpoint::IncomingSession);
newtype!(SessionRequest => wtransport::endpoint::SessionRequest);
newtype!(OpeningBiStream => wtransport::stream::OpeningBiStream);
newtype!(OpeningUniStream => wtransport::stream::OpeningUniStream);
newtype_pin!(SendStream => wtransport::SendStream);
newtype_pin!(RecvStream => wtransport::RecvStream);
newtype!(StreamErrorCode => u32);
newtype!(Datagram => wtransport::datagram::Datagram);

/// Expose the [`Session`] as a type alias for [`Connection`], as `wtransport`
/// does not use the session terminology but it might be convenient for
/// the `xwt` users.
pub type Session = Connection;

/// The read error.
#[derive(Debug, thiserror::Error)]
pub enum StreamReadError {
    /// The `wtransport` read call failed.
    #[error("stream read: {0}")]
    Read(wtransport::error::StreamReadError),

    /// The stream was closed without an error.
    #[error("stream closed")]
    Closed,
}

/// The write error.
#[derive(Debug, thiserror::Error)]
pub enum StreamWriteError {
    /// The write was called with a zero-size write buffer.
    #[error("zero size write buffer")]
    ZeroSizeWriteBuffer,

    /// The `wtransport` write call failed.
    #[error("stream write: {0}")]
    Write(wtransport::error::StreamWriteError),
}
