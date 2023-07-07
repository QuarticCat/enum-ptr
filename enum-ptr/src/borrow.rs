use core::ops::Deref;

use crate::Compact;

/// Types that can be borrowed from [`Compact`].
pub trait CompactBorrow
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
    type Target;

    unsafe fn deref<'a>(&self) -> &'a Self::Target;
}

unsafe impl<T> FieldDeref for &T {
    type Target = T;

    unsafe fn deref<'a>(&self) -> &'a Self::Target {
        &*(Deref::deref(self) as *const _)
    }
}

unsafe impl<T> FieldDeref for &mut T {
    type Target = T;

    unsafe fn deref<'a>(&self) -> &'a Self::Target {
        &*(Deref::deref(self) as *const _)
    }
}

#[cfg(feature = "alloc")]
mod alloc_impl {
    use super::*;

    use alloc::boxed::Box;
    use alloc::rc::Rc;
    use alloc::sync::Arc;

    unsafe impl<T> FieldDeref for Box<T> {
        type Target = T;

        unsafe fn deref<'a>(&self) -> &'a Self::Target {
            &*(Deref::deref(self) as *const _)
        }
    }

    unsafe impl<T> FieldDeref for Rc<T> {
        type Target = T;

        unsafe fn deref<'a>(&self) -> &'a Self::Target {
            &*(Deref::deref(self) as *const _)
        }
    }

    unsafe impl<T> FieldDeref for Arc<T> {
        type Target = T;

        unsafe fn deref<'a>(&self) -> &'a Self::Target {
            &*(Deref::deref(self) as *const _)
        }
    }
}
