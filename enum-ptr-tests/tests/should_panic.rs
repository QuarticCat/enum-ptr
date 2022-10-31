use enum_ptr::EnumPtr;

#[test]
#[should_panic(expected = "`Foo::B` has no enough alignment")]
fn no_enough_alignment() {
    #[derive(Debug, EnumPtr)]
    #[repr(C, usize)]
    enum Foo<'a, 'b> {
        A(&'a i32),
        B(&'b i8),
    }

    dbg!(Foo::from(CompactFoo::from(Foo::A(&42))));
    dbg!(Foo::from(CompactFoo::from(Foo::B(&43))));
}
