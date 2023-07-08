#![cfg(feature = "alloc")]
#![allow(dead_code, clippy::disallowed_names)]

use enum_ptr::*;

/* ----- lib code ----- */

#[doc(hidden)]
unsafe fn get_ref_helper<T, U>(
    compact: &Compact<T>,
    f: impl FnOnce(&T) -> Option<&U>,
) -> Option<<U as FieldDeref>::Target<'_>>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
    U: FieldDeref,
{
    compact.map_ref(|tmp| f(tmp).map(|tmp| tmp.force_deref()))
}

/* ----- user code ----- */

#[test]
fn simplest() {
    #[derive(EnumPtr)]
    #[repr(C, usize)]
    enum Foo {
        A(Box<i32>),
        B(Box<u32>),
    }

    let foo: Compact<_> = Foo::A(Box::new(1)).into();

    /* ----- macro code begin ----- */

    let result = unsafe {
        get_ref_helper(&foo, |tmp| match tmp {
            Foo::A(inner) => Some(inner),
            _ => None,
        })
    };

    /* ----- macro code end ----- */

    assert_eq!(result, Some(&1));

    // uncomment to check lifetime correctness
    // drop(foo);
    // dbg!(result);
}
