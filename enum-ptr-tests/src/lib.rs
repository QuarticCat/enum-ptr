#[macro_export]
macro_rules! test {
    ($name:ident, $type:ident, $compact_type:ident, $val:expr) => {
        #[test]
        fn $name() {
            let c = $compact_type::from($val);
            let _ = $type::from(c);
        }
    };
}

#[macro_export]
macro_rules! test_foo {
    ($val:expr) => {
        $crate::test!(test_foo, Foo, CompactFoo, $val);
    };
}
