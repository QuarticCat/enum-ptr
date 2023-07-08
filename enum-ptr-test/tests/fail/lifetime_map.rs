#![allow(dead_code)]

use enum_ptr::{Compact, EnumPtr};

#[derive(EnumPtr)]
#[repr(C, usize)]
enum Foo<'a, 'b> {
    A(&'a i32),
    B(&'b i32),
}

fn test_map_ref() {
    let foo: Compact<_> = Foo::A(&1).into();
    foo.map_ref(|f| match f {
        Foo::A(r) | Foo::B(r) => r,
    });
}

unsafe fn test_map_mut() {
    let mut foo: Compact<_> = Foo::A(&1).into();
    foo.map_mut(|f| match f {
        Foo::A(r) | Foo::B(r) => r,
    });
}

fn main() {}
