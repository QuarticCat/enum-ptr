#![allow(dead_code)]

use enum_ptr::EnumPtrCopy;

#[derive(EnumPtrCopy)]
#[repr(C, usize)]
enum Foo<'a, 'b> {
    A(&'a i32),
    B(&'b i32),
}

fn main() {}
