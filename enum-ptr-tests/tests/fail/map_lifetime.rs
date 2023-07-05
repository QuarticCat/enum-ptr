#![allow(dead_code)]

use enum_ptr::{Compact, EnumPtr};

#[derive(EnumPtr)]
#[repr(C, usize)]
enum Foo<'a, 'b> {
    A(&'a i32),
    B(&'b i32),
}

fn main() {
    let mut foo: Compact<_> = Foo::A(&1).into();
    foo.map_ref(|f| match f {
        Foo::A(r) | Foo::B(r) => r,
    });
    foo.map_mut(|f| match f {
        Foo::A(r) | Foo::B(r) => r,
    });
}
