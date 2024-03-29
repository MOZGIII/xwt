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
    };
}

newtype!(Endpoint<Side> => wtransport::Endpoint<Side>);
newtype!(Connection => wtransport::Connection);
newtype!(IncomingSession => wtransport::endpoint::IncomingSession);
newtype!(SessionRequest => wtransport::endpoint::SessionRequest);
newtype!(OpeningBiStream => wtransport::stream::OpeningBiStream);
newtype!(OpeningUniStream => wtransport::stream::OpeningUniStream);
newtype!(SendStream => wtransport::SendStream);
newtype!(RecvStream => wtransport::RecvStream);
newtype!(Datagram => wtransport::datagram::Datagram);
