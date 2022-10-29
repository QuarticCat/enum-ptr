#[macro_export]
macro_rules! static_assert {
    ($($arg:tt)+) => {
        #[allow(clippy::assertions_on_constants)]
        const _: () = assert!($($arg)+);
    };
}
