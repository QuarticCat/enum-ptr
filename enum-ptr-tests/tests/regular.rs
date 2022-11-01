use enum_ptr::{Compact, EnumPtr};

#[test]
fn all() {
    #[derive(EnumPtr, Debug, PartialEq, Eq, Clone)]
    #[repr(C, usize)]
    enum Foo<'a, 'b> {
        A(&'a u64),
        B { ptr: &'b i64 },
        C(Option<Box<i64>>),
        D(),
        E {},
        F,
    }

    let test = |f: Foo| assert_eq!(Foo::from(Compact::from(f.clone())), f);

    test(Foo::A(&0));
    test(Foo::B { ptr: &1 });
    test(Foo::C(None));
    test(Foo::C(Some(Box::new(2))));
    test(Foo::D());
    test(Foo::E {});
    test(Foo::F);
}
