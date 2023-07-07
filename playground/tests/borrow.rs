#![cfg(feature = "alloc")]
#![allow(dead_code)]

use core::mem::transmute;
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

#[doc(hidden)]
unsafe trait FieldDeref {
    type Target<'a>
    where
        Self: 'a;

    fn deref(&self) -> Self::Target<'_>;
}

unsafe impl<T> FieldDeref for Box<T> {
    type Target<'a> = &'a T
    where
        Self: 'a;

    fn deref(&self) -> Self::Target<'_> {
        Deref::deref(self)
    }
}

unsafe impl<T> FieldDeref for Option<Box<T>> {
    type Target<'a> = Option<&'a T>
    where
        Self: 'a;

    fn deref(&self) -> Self::Target<'_> {
        self.as_deref()
    }
}

unsafe impl<T> FieldDeref for &T {
    type Target<'a> = &'a T
    where
        Self: 'a;

    fn deref(&self) -> Self::Target<'_> {
        Deref::deref(self)
    }
}

/* ----- user code ----- */

#[test]
fn simplest() {
    #[derive(EnumPtr, Debug)]
    #[repr(C, usize)]
    enum Foo {
        A(Box<i32>),
        B(Box<u32>),
    }

    /* ----- derived code begin ----- */

    #[repr(C, usize)]
    enum FooRef<'enum_ptr> {
        A(<Box<i32> as FieldDeref>::Target<'enum_ptr>),
        B(<Box<u32> as FieldDeref>::Target<'enum_ptr>),
    }

    impl CompactBorrow for Foo {
        type Target<'enum_ptr> = FooRef<'enum_ptr>
        where
            Self: 'enum_ptr;

        fn borrow(compact: &Compact<Self>) -> Self::Target<'_> {
            unsafe {
                compact.map_ref(|f| match f {
                    Self::A(inner) => Self::Target::A(transmute(FieldDeref::deref(inner))),
                    Self::B(inner) => Self::Target::B(transmute(FieldDeref::deref(inner))),
                })
            }
        }
    }

    /* ----- derived code end ----- */

    let compact_foo: Compact<_> = Foo::A(Box::new(0)).into();
    let foo_ref = CompactBorrow::borrow(&compact_foo);
    let value = match foo_ref {
        FooRef::A(inner) => *inner,
        _ => unreachable!(),
    };
    assert_eq!(value, 0);
}

#[test]
fn with_option() {
    #[derive(EnumPtr, Debug)]
    #[repr(C, usize)]
    enum Foo {
        A(Option<Box<i32>>),
        B(Option<Box<u32>>),
    }

    /* ----- derived code begin ----- */

    #[repr(C, usize)]
    enum FooRef<'enum_ptr> {
        A(<Option<Box<i32>> as FieldDeref>::Target<'enum_ptr>),
        B(<Option<Box<u32>> as FieldDeref>::Target<'enum_ptr>),
    }

    impl CompactBorrow for Foo {
        type Target<'enum_ptr> = FooRef<'enum_ptr>
        where
            Self: 'enum_ptr;

        fn borrow(compact: &Compact<Self>) -> Self::Target<'_> {
            unsafe {
                compact.map_ref(|f| match f {
                    Self::A(inner) => Self::Target::A(transmute(FieldDeref::deref(inner))),
                    Self::B(inner) => Self::Target::B(transmute(FieldDeref::deref(inner))),
                })
            }
        }
    }

    /* ----- derived code end ----- */

    let compact_foo: Compact<_> = Foo::A(Some(Box::new(0))).into();
    let foo_ref = CompactBorrow::borrow(&compact_foo);
    let value = match foo_ref {
        FooRef::A(Some(inner)) => *inner,
        _ => unreachable!(),
    };
    assert_eq!(value, 0);
}

#[test]
fn with_lifetime() {
    #[derive(EnumPtr, Debug)]
    #[repr(C, usize)]
    enum Foo<'a, 'b> {
        A(&'a i32),
        B(&'b u32),
    }

    /* ----- derived code begin ----- */

    #[repr(C, usize)]
    enum FooRef<'enum_ptr, 'a, 'b>
    where
        Foo<'a, 'b>: 'enum_ptr,
    {
        A(<&'a i32 as FieldDeref>::Target<'enum_ptr>),
        B(<&'b u32 as FieldDeref>::Target<'enum_ptr>),
    }

    impl<'a, 'b> CompactBorrow for Foo<'a, 'b> {
        type Target<'enum_ptr> = FooRef<'enum_ptr, 'a, 'b>
        where
            Self: 'enum_ptr;

        fn borrow(compact: &Compact<Self>) -> Self::Target<'_> {
            unsafe {
                compact.map_ref(|f| match f {
                    Self::A(inner) => Self::Target::A(transmute(FieldDeref::deref(inner))),
                    Self::B(inner) => Self::Target::B(transmute(FieldDeref::deref(inner))),
                })
            }
        }
    }

    /* ----- derived code end ----- */

    let num = 0;
    let compact_foo: Compact<_> = Foo::A(&num).into();
    let foo_ref = CompactBorrow::borrow(&compact_foo);
    let value = match foo_ref {
        FooRef::A(inner) => *inner,
        _ => unreachable!(),
    };
    assert_eq!(value, 0);
}

#[test]
fn with_generic_type() {
    #[derive(EnumPtr, Debug)]
    #[repr(C, usize)]
    enum Foo<'a, T, U: Aligned + FieldDeref> {
        A(&'a T),
        B(U),
    }

    /* ----- derived code begin ----- */

    #[repr(C, usize)]
    enum FooRef<'enum_ptr, 'a, T, U: Aligned + FieldDeref>
    where
        Foo<'a, T, U>: 'enum_ptr,
    {
        A(<&'a T as FieldDeref>::Target<'enum_ptr>),
        B(<U as FieldDeref>::Target<'enum_ptr>),
    }

    impl<'a, T, U: Aligned + FieldDeref> CompactBorrow for Foo<'a, T, U> {
        type Target<'enum_ptr> = FooRef<'enum_ptr, 'a, T, U>
        where
            Self: 'enum_ptr;

        fn borrow(compact: &Compact<Self>) -> Self::Target<'_> {
            unsafe {
                compact.map_ref(|f| match f {
                    Self::A(inner) => Self::Target::A(transmute(FieldDeref::deref(inner))),
                    Self::B(inner) => Self::Target::B(transmute(FieldDeref::deref(inner))),
                })
            }
        }
    }

    /* ----- derived code end ----- */

    let num = 0;
    let compact_foo: Compact<_> = Foo::<i32, &u32>::A(&num).into();
    let foo_ref = CompactBorrow::borrow(&compact_foo);
    let value = match foo_ref {
        FooRef::A(inner) => *inner,
        _ => unreachable!(),
    };
    assert_eq!(value, 0);
}
