#![allow(dead_code)]

use enum_ptr::{Compact, EnumPtr};

#[derive(EnumPtr)]
#[enum_ptr(borrow, borrow_mut)]
#[repr(C, usize)]
enum Foo<'a, 'b> {
    A(Option<&'a mut i32>),
    B(Option<&'b mut i32>),
}

fn test_get_ref() {
    let foo: Compact<_> = Foo::A(None).into();
    let foo_ref = foo.borrow();
    drop(foo);
    drop(foo_ref);
}

fn test_get_mut() {
    let mut foo: Compact<_> = Foo::A(None).into();
    let foo_mut = foo.borrow_mut();
    drop(foo);
    drop(foo_mut);
}

fn main() {}
