//! Problem: cannot support non-ptr types like `Option<&T>`

#![cfg(feature = "alloc")]
#![allow(dead_code, clippy::missing_safety_doc)]

use core::ops::Deref;

use enum_ptr::*;

/* ----- lib code ----- */

trait CompactBorrow
where
    Self: From<Compact<Self>>,
    Compact<Self>: From<Self>,
{
    type Target<'a>
    where
        Self: 'a;

    fn borrow(compact: &Compact<Self>) -> Self::Target<'_>;
}

unsafe trait FieldDeref {
    type Target;

    unsafe fn deref<'a>(&self) -> &'a Self::Target;
}

unsafe impl<T> FieldDeref for Box<T> {
    type Target = T;

    unsafe fn deref<'a>(&self) -> &'a Self::Target {
        &*(Deref::deref(self) as *const _)
    }
}

unsafe impl<T> FieldDeref for &T {
    type Target = T;

    unsafe fn deref<'a>(&self) -> &'a Self::Target {
        &*(Deref::deref(self) as *const _)
    }
}

/* ----- user code ----- */

// simplest
#[test]
fn case1() {
    #[derive(EnumPtr, Debug)]
    #[repr(C, usize)]
    enum Foo {
        A(Box<i32>),
        B(Box<u32>),
    }

    /* ----- derived code begin ----- */

    #[repr(C, usize)]
    enum FooRef<'enum_ptr> {
        A(&'enum_ptr <Box<i32> as FieldDeref>::Target),
        B(&'enum_ptr <Box<u32> as FieldDeref>::Target),
    }

    impl CompactBorrow for Foo {
        type Target<'enum_ptr> = FooRef<'enum_ptr>
        where
            Self: 'enum_ptr;

        fn borrow(compact: &Compact<Self>) -> Self::Target<'_> {
            unsafe {
                compact.map_ref(|f| match f {
                    Self::A(inner) => Self::Target::A(FieldDeref::deref(inner)),
                    Self::B(inner) => Self::Target::B(FieldDeref::deref(inner)),
                })
            }
        }
    }

    /* ----- derived code end ----- */

    let compact_foo: Compact<_> = Foo::A(Box::new(0)).into();
    let foo_ref = CompactBorrow::borrow(&compact_foo);
    let value = match foo_ref {
        FooRef::A(inner) => *inner as i64 + 1,
        FooRef::B(inner) => *inner as i64 + 2,
    };
    assert_eq!(value, 1);
}

// with lifetime variables
#[test]
fn case2() {
    #[derive(EnumPtr, Debug)]
    #[repr(C, usize)]
    enum Foo<'a, 'b> {
        A(&'a i32),
        B(&'b u32),
    }

    /* ----- derived code begin ----- */

    #[repr(C, usize)]
    enum FooRef<'enum_ptr, 'a, 'b> {
        A(&'enum_ptr <&'a i32 as FieldDeref>::Target),
        B(&'enum_ptr <&'b u32 as FieldDeref>::Target),
    }

    impl<'a, 'b> CompactBorrow for Foo<'a, 'b> {
        type Target<'enum_ptr> = FooRef<'enum_ptr, 'a, 'b>
        where
            Self: 'enum_ptr;

        fn borrow(compact: &Compact<Self>) -> Self::Target<'_> {
            unsafe {
                compact.map_ref(|f| match f {
                    Self::A(inner) => Self::Target::A(FieldDeref::deref(inner)),
                    Self::B(inner) => Self::Target::B(FieldDeref::deref(inner)),
                })
            }
        }
    }

    /* ----- derived code end ----- */

    let num = 0;
    let compact_foo: Compact<_> = Foo::A(&num).into();
    let foo_ref = CompactBorrow::borrow(&compact_foo);
    let value = match foo_ref {
        FooRef::A(inner) => *inner as i64 + 1,
        FooRef::B(inner) => *inner as i64 + 2,
    };
    assert_eq!(value, 1);
}

// with type variables
#[test]
fn case3() {
    #[derive(EnumPtr, Debug)]
    #[repr(C, usize)]
    enum Foo<'a, T, U: Aligned + FieldDeref> {
        A(&'a T),
        B(U),
    }

    /* ----- derived code begin ----- */

    #[repr(C, usize)]
    enum FooRef<'enum_ptr, 'a, T, U: Aligned + FieldDeref> {
        A(&'enum_ptr <&'a T as FieldDeref>::Target),
        B(&'enum_ptr <U as FieldDeref>::Target),
    }

    impl<'a, T, U: Aligned + FieldDeref> CompactBorrow for Foo<'a, T, U> {
        type Target<'enum_ptr> = FooRef<'enum_ptr, 'a, T, U>
        where
            Self: 'enum_ptr;

        fn borrow(compact: &Compact<Self>) -> Self::Target<'_> {
            unsafe {
                compact.map_ref(|f| match f {
                    Self::A(inner) => Self::Target::A(FieldDeref::deref(inner)),
                    Self::B(inner) => Self::Target::B(FieldDeref::deref(inner)),
                })
            }
        }
    }

    /* ----- derived code end ----- */

    let num = 0;
    let compact_foo: Compact<_> = Foo::<i32, &u32>::A(&num).into();
    let foo_ref = CompactBorrow::borrow(&compact_foo);
    let value = match foo_ref {
        FooRef::A(inner) => *inner as i64 + 1,
        FooRef::B(inner) => *inner as i64 + 2,
    };
    assert_eq!(value, 1);
}
