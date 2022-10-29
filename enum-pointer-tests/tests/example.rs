#![allow(dead_code)]

use enum_pointer::EnumPointer;

#[derive(EnumPointer)]
enum Foo<'a> {
    A(&'a str),
}
