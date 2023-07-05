#![allow(dead_code)]

use enum_ptr::EnumPtr;

#[derive(EnumPtr)]
#[repr(C, usize)]
enum Foo<'a, 'b> {
    A(&'a i32),
    B(&'b i32, *const i32),
}

#[derive(EnumPtr)]
#[repr(C, usize)]
enum Bar<'a> {
    A(&'a i32),
    B,
}

fn main() {}
