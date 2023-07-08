#![allow(dead_code)]

use enum_ptr::{get_mut, get_ref, Compact, EnumPtr};

#[derive(EnumPtr)]
#[repr(C, usize)]
enum Foo<'a, 'b> {
    A(Option<&'a mut i32>),
    B(Option<&'b mut i32>),
}

fn test_get_ref() {
    let foo: Compact<_> = Foo::A(None).into();
    let foo_ref = get_ref!(foo, Foo::A);
    drop(foo);
    drop(foo_ref);
}

fn test_get_mut() {
    let mut foo: Compact<_> = Foo::A(None).into();
    let foo_mut = get_mut!(foo, Foo::A);
    drop(foo);
    drop(foo_mut);
}

fn main() {}
