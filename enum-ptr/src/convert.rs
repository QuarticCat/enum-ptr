use core::mem::{transmute, transmute_copy, ManuallyDrop};

use crate::{Compact, CompactCopy};

impl<T> From<Compact<T>> for CompactCopy<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
    T: From<CompactCopy<T>> + Copy,
    CompactCopy<T>: From<T>,
{
    #[inline]
    fn from(value: Compact<T>) -> Self {
        unsafe { transmute(value) }
    }
}

impl<T> From<CompactCopy<T>> for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
    T: From<CompactCopy<T>> + Copy,
    CompactCopy<T>: From<T>,
{
    #[inline]
    fn from(value: CompactCopy<T>) -> Self {
        unsafe { transmute(value) }
    }
}

#[repr(C)]
struct PtrRepr(pub usize, pub *const u8);

#[doc(hidden)]
#[inline]
pub unsafe fn compact<T>(value: T) -> *const u8 {
    let PtrRepr(tag, ptr) = transmute_copy(&ManuallyDrop::new(value));
    ptr.wrapping_add(tag)
}

#[doc(hidden)]
#[inline]
pub unsafe fn extract<T>(value: *const u8, mask: usize) -> T {
    let tag = value as usize & mask;
    let ptr = value.wrapping_sub(tag);
    transmute_copy(&PtrRepr(tag, ptr))
}
