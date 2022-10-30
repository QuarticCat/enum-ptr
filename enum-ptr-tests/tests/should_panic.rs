#![allow(dead_code)]

use enum_ptr::EnumPtr;

#[test]
#[should_panic(expected = "`Foo::B` has no enough alignment")]
fn no_enough_alignment() {
    #[derive(EnumPtr)]
    #[repr(C, usize)]
    enum Foo<'a, 'b> {
        A(&'a i32),
        B(&'b i8),
    }

    let _ = CompactFoo::from(Foo::A(&0));
}
