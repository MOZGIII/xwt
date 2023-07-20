macro_rules! newtype {
    ($name:ident => $wrapped_type:path) => {
        pub struct $name(pub $wrapped_type);

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($name)).finish()
            }
        }
    };
    ($name:ident < $($generics:tt),* > => $wrapped_type:path) => {
        pub struct $name<$($generics)*>(pub $wrapped_type);

        impl<$($generics)*> std::fmt::Debug for $name<$($generics)*> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($name)).finish()
            }
        }
    };
}

newtype!(Endpoint<Side> => wtransport::Endpoint<Side>);
newtype!(Connection => wtransport::Connection);
newtype!(SessionRequest => wtransport::endpoint::SessionRequest);
newtype!(OpeningBiStream => wtransport::stream::OpeningBiStream);
newtype!(OpeningUniStream => wtransport::stream::OpeningUniStream);
