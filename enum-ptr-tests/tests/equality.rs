use enum_ptr::{Compact, EnumPtr, Unit, UNIT};

#[test]
fn equality() {
    #[derive(EnumPtr, Debug, PartialEq, Eq, Clone)]
    #[repr(C, usize)]
    enum Foo<'a, 'b> {
        A(&'a u64),
        B {
            ptr: &'b i64,
        },
        #[cfg(feature = "alloc")]
        C(Option<Box<i64>>),
        D(Unit),
    }

    let test = |f: Foo| assert_eq!(Foo::from(Compact::from(f.clone())), f);

    test(Foo::A(&0));
    test(Foo::B { ptr: &1 });
    #[cfg(feature = "alloc")]
    {
        test(Foo::C(None));
        test(Foo::C(Some(Box::new(2))));
    }
    test(Foo::D(UNIT));
}
