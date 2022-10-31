use enum_ptr::EnumPtr;

#[test]
fn unit_variants() {
    #[derive(Debug, EnumPtr)]
    #[repr(C, usize)]
    enum Foo<'a> {
        A,
        B(&'a i32),
    }

    dbg!(Foo::from(CompactFoo::from(Foo::A)));
    dbg!(Foo::from(CompactFoo::from(Foo::B(&42))));
}
