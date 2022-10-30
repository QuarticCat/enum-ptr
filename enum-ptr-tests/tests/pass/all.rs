#![allow(dead_code)]

use enum_ptr::EnumPtr;

#[derive(EnumPtr)]
#[repr(C, usize)]
enum Foo<'a, 'b> {
    A(&'a i32),
    B(&'b mut i32),
    C(Option<Box<i32>>),
    D(),
    E {},
    F,
}

fn main() {}
