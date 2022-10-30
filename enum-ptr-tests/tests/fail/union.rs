#![allow(dead_code)]

use enum_ptr::EnumPtr;

#[derive(EnumPtr)]
union Foo<'a, 'b> {
    a: &'a i32,
    b: &'b i32,
}

fn main() {}
