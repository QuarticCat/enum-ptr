#![cfg(feature = "alloc")]
#![allow(dead_code, clippy::disallowed_names)]

use core::marker::PhantomData;
use core::mem::{transmute, transmute_copy, ManuallyDrop};

/* ----- lib code ----- */

// inner representation

#[repr(transparent)]
pub struct CompactInner<T: Compactable<Target = Self>> {
    data: *const u8,
    marker: PhantomData<T>,
}

impl<T: Compactable<Target = Self>> Drop for CompactInner<T> {
    #[inline]
    fn drop(&mut self) {
        drop(T::extract(Self { ..*self }));
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CompactInnerCopy<T: Compactable<Target = Self> + Copy> {
    data: *const u8,
    marker: PhantomData<T>,
}

// traits

mod private {
    use super::*;

    pub trait Sealed {}

    impl<T: Compactable<Target = Self>> Sealed for CompactInner<T> {}

    impl<T: Compactable<Target = Self> + Copy> Sealed for CompactInnerCopy<T> {}
}

pub trait Compactable {
    type Target: private::Sealed;

    fn compact(self) -> Self::Target;

    fn extract(value: Self::Target) -> Self;
}

// public compact struct

#[repr(transparent)]
pub struct Compact<T: Compactable> {
    inner: T::Target,
}

impl<T: Compactable> Compact<T> {
    unsafe fn temp_extract(&self) -> ManuallyDrop<T> {
        ManuallyDrop::new(T::extract(transmute_copy(&self.inner)))
    }
}

impl<T: Compactable + Clone> Clone for Compact<T> {
    fn clone(&self) -> Self {
        Self {
            inner: unsafe { T::clone(&self.temp_extract()).compact() },
        }
    }
}

impl<T: Compactable<Target = CompactInnerCopy<T>> + Copy> Copy for Compact<T> {}

/* ----- user code ----- */

#[test]
fn simplest() {
    #[derive(Clone, Copy)]
    #[repr(C, usize)]
    enum Foo<'a, 'b> {
        A(&'a i32),
        B(&'b u32),
    }

    /* ----- derived code begin ----- */

    impl<'a, 'b> Compactable for Foo<'a, 'b> {
        type Target = CompactInnerCopy<Self>;

        fn compact(self) -> Self::Target {
            // asserts
            unsafe { transmute(enum_ptr::compact(self)) }
        }

        fn extract(value: Self::Target) -> Self {
            unsafe { enum_ptr::extract(transmute(value), 0b1) }
        }
    }

    /* ----- derived code end ----- */

    let foo = Foo::A(&1).compact();
    let _ = foo;
    let _ = foo;
}
