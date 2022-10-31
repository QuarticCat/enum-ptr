#![allow(dead_code)]

use enum_ptr::{Compact, EnumPtr};

#[derive(EnumPtr)]
#[repr(C, usize)]
enum Foo<'a, 'b, T> {
    A(&'a i64),
    B { ptr: &'b mut T },
    C(Option<Box<i64>>),
    D(),
    E {},
    F,
}

fn main() {
    let _ = Compact::<Foo<u64>>::from(Foo::<u64>::A(&0));
}
