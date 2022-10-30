#![allow(dead_code)]

use enum_ptr::EnumPtr;

#[derive(EnumPtr)]
#[repr(C, usize)]
enum Foo<'a, T> {
    A(&'a i64),
    B { ptr: *mut T },
    C(Option<Box<i64>>),
    D(),
    E {},
    F,
}

fn main() {
    let _ = CompactFoo::from(Foo::<u64>::A(&0));
}
