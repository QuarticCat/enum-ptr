#![allow(dead_code)]

use enum_ptr::{EnumPtr, Unit};

#[derive(EnumPtr)]
#[enum_ptr(borrow_mut)]
#[repr(C, usize)]
enum Foo<'a> {
    A(&'a mut i32),
    B(Unit),
}

#[derive(EnumPtr)]
#[enum_ptr(borrow_mut)]
#[repr(C, usize)]
enum Bar<'a> {
    A(&'a mut i32),
    #[enum_ptr(skip)]
    B(Unit),
}

#[derive(EnumPtr)]
#[enum_ptr(borrow_mut(rename = "RefMutBaz"))]
#[repr(C, usize)]
enum Baz<'a> {
    A(&'a mut i32),
    #[enum_ptr(skip_borrow_mut)]
    B(Unit),
}

fn main() {}
