#![allow(dead_code)]

use enum_ptr::EnumPtr;

#[derive(EnumPtr)]
#[repr(C, usize)]
enum Foo<'a, 'b> {
    A(&'a i32),
    B { ptr: &'b i32 },
}

fn main() {}
