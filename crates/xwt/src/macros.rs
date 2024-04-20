//! Macros for the [`xwt`] crate.

/// Invoke a macro with a comma-separated list of all material types.
#[macro_export]
macro_rules! for_all_material_types {
    (
        $macro:ident
    ) => {
        $macro![
            Endpoint<T>,
            Connecting<T>,
            Accepting<T>,
            Connection<T>,
            Request<T>,
            OpeningBiStream<T>,
            OpeningUniStream<T>,
            SendStream<T>,
            RecvStream<T>,
            Datagram<T>,
        ];
    };
}
