use core::ops::DerefMut;

use crate::Compact;

/// Types that can be mutably borrowed from [`Compact`].
pub trait CompactBorrowMut
where
    Self: From<Compact<Self>>,
    Compact<Self>: From<Self>,
{
    type Target<'a>
    where
        Self: 'a;

    fn borrow_mut(compact: &mut Compact<Self>) -> Self::Target<'_>;
}

#[doc(hidden)]
unsafe trait FieldDerefMut {
    type Target;

    unsafe fn deref_mut<'a>(&mut self) -> &'a mut Self::Target;
}

unsafe impl<T> FieldDerefMut for &mut T {
    type Target = T;

    unsafe fn deref_mut<'a>(&mut self) -> &'a mut Self::Target {
        &mut *(DerefMut::deref_mut(self) as *mut _)
    }
}

#[cfg(feature = "alloc")]
mod alloc_impl {
    use super::*;

    use alloc::boxed::Box;

    unsafe impl<T> FieldDerefMut for Box<T> {
        type Target = T;

        unsafe fn deref_mut<'a>(&mut self) -> &'a mut Self::Target {
            &mut *(DerefMut::deref_mut(self) as *mut _)
        }
    }
}
