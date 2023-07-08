#![allow(dead_code)]

mod lib_code {
    pub trait Expose {
        type Target;
    }
}

mod user_code {
    use super::lib_code::Expose;

    pub struct Foo;

    // try remove `pub`
    pub struct FooRef;

    impl Expose for Foo {
        type Target = FooRef;
    }
}
