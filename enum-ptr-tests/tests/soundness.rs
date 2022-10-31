use enum_ptr::{Compact, EnumPtr};

#[test]
fn unit_variants() {
    #[derive(Debug, EnumPtr)]
    #[repr(C, usize)]
    enum Foo<'a> {
        A,
        B(&'a i32),
    }

    dbg!(Foo::from(Compact::<Foo>::from(Foo::A)));
    dbg!(Foo::from(Compact::<Foo>::from(Foo::B(&42))));
}
