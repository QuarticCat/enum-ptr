#![allow(dead_code)]

use enum_pointer::EnumPointer;

#[derive(EnumPointer)]
enum Foo<'a, 'b> {
    A(&'a i32),
    B(&'b i32),
}
