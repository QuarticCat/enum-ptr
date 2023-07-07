#![allow(dead_code)]

use enum_ptr::{EnumPtr, Unit};

#[derive(EnumPtr)]
#[enum_ptr(borrow)]
#[repr(C, usize)]
enum Foo<'a> {
    A(&'a i32),
    B(Unit),
}

#[derive(EnumPtr)]
#[enum_ptr(borrow)]
#[repr(C, usize)]
enum Bar<'a> {
    A(&'a i32),
    #[enum_ptr(skip)]
    B(Unit),
}

#[derive(EnumPtr)]
#[enum_ptr(borrow(rename = "RefBaz"))]
#[repr(C, usize)]
enum Baz<'a> {
    A(&'a i32),
    #[enum_ptr(skip_borrow)]
    B(Unit),
}

fn main() {}
