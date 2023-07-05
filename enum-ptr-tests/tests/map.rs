use enum_ptr::{Compact, EnumPtr};

#[derive(EnumPtr, Debug)]
#[repr(C, usize)]
enum Foo<'a, 'b> {
    A(&'a i32),
    B(&'b i32),
}

fn main() {
    let mut foo_a: Compact<_> = Foo::A(&1).into();
    foo_a.map_ref(|f| println!("{f:?}"));
    foo_a.map_mut(|f| println!("{f:?}"));

    let mut foo_b: Compact<_> = Foo::B(&2).into();
    foo_b.map_ref(|f| println!("{f:?}"));
    foo_b.map_mut(|f| println!("{f:?}"));
}
