#[cfg(feature = "trace-calls")]
#[macro_export]
macro_rules! trace_call {
    () => {
        fn r#fn() {}
        tracing::trace!(
            message = "entering",
            r#fn = %$crate::trace_calls::type_name_of(r#fn)
                .strip_suffix("::{{closure}}::fn")
                .unwrap()
        );
    };
}

#[cfg(feature = "trace-calls")]
pub(crate) fn type_name_of<T>(_: T) -> &'static str {
    core::any::type_name::<T>()
}

#[cfg(not(feature = "trace-calls"))]
#[macro_export]
macro_rules! trace_call {
    () => {};
}
