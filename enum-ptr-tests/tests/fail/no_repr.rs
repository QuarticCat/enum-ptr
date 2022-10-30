#![allow(dead_code)]

use enum_ptr::EnumPtr;

#[derive(EnumPtr)]
enum Foo<'a, 'b> {
    A(&'a i32),
    B(&'b i32),
}

fn main() {}
