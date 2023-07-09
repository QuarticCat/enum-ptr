use core::mem::{transmute_copy, ManuallyDrop};

use crate::{Compact, CompactInner, CompactInnerCopy};

#[repr(C)]
struct PtrRepr(pub usize, pub *const u8);

mod private {
    use super::*;

    pub trait Sealed {}

    impl<T: Compactable<Inner = Self>> Sealed for CompactInner<T> {}

    impl<T: Compactable<Inner = Self> + Copy> Sealed for CompactInnerCopy<T> {}
}

#[doc(hidden)]
pub unsafe trait Compactable: Sized {
    type Inner: private::Sealed;

    const MASK: usize;

    #[inline]
    fn compact(self) -> Compact<Self> {
        let inner = self.compact_inner();
        Compact { inner }
    }

    #[inline]
    fn compact_inner(self) -> Self::Inner {
        let PtrRepr(tag, ptr) = unsafe { transmute_copy(&ManuallyDrop::new(self)) };
        unsafe { transmute_copy(&ptr.wrapping_add(tag)) }
    }

    #[inline]
    fn extract(value: Compact<Self>) -> Self {
        Self::extract_inner(value.inner)
    }

    #[inline]
    fn extract_inner(value: Self::Inner) -> Self {
        let value: *const u8 = unsafe { transmute_copy(&ManuallyDrop::new(value)) };
        let tag = value as usize & Self::MASK;
        let ptr = value.wrapping_sub(tag);
        unsafe { transmute_copy(&PtrRepr(tag, ptr)) }
    }
}
