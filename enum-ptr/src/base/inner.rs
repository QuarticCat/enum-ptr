use core::marker::PhantomData;
use core::mem::ManuallyDrop;

use crate::Compactable;

#[doc(hidden)]
#[repr(transparent)]
pub struct CompactInner<T: Compactable<Inner = Self>> {
    _data: *const u8,
    marker: PhantomData<T>,
}

impl<T: Compactable<Inner = Self>> Drop for CompactInner<T> {
    #[inline]
    fn drop(&mut self) {
        drop(T::extract_inner(Self { ..*self }));
    }
}

impl<T: Compactable<Inner = Self> + Clone> Clone for CompactInner<T> {
    #[inline]
    fn clone(&self) -> Self {
        T::clone(&ManuallyDrop::new(T::extract_inner(Self { ..*self }))).compact_inner()
    }
}

#[doc(hidden)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CompactInnerCopy<T: Compactable<Inner = Self> + Copy> {
    _data: *const u8,
    marker: PhantomData<T>,
}
