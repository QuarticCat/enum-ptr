#![allow(dead_code)]

use enum_ptr::EnumPtr;

#[derive(EnumPtr)]
#[repr(C, usize)]
enum Foo<'a, 'b> {
    A(&'a i64),
    B(&'b mut i64),
    C(Option<Box<i64>>),
    D(),
    E {},
    F,
}

fn main() {}
