use core::mem::transmute;
use core::ops::Deref;

use crate::Compact;

/// Types that can be borrowed from [`Compact`]. Typically derived from
/// [`EnumPtr`](crate::EnumPtr).
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

/// Types that can be used by [`get_ref`](crate::get_ref) and to derive
/// [`CompactBorrow`].
///
/// It's like [`Deref`] but with flexible targets and strict constraints.
///
/// # Safety
///
/// `T` must not `deref` to something that points to its own memory.
///
/// A counter-example is `ManuallyDrop<T>`, which will `deref` to `&T`.
pub unsafe trait FieldDeref {
    type Target<'a>
    where
        Self: 'a;

    fn deref(&self) -> Self::Target<'_>;

    #[doc(hidden)]
    #[inline]
    unsafe fn force_deref<'a>(&self) -> Self::Target<'a> {
        transmute(self.deref())
    }
}

unsafe impl<T> FieldDeref for &T {
    type Target<'a> = &'a T
    where
        Self: 'a;

    #[inline]
    fn deref(&self) -> Self::Target<'_> {
        Deref::deref(self)
    }
}

unsafe impl<T> FieldDeref for &mut T {
    type Target<'a> = &'a T
    where
        Self: 'a;

    #[inline]
    fn deref(&self) -> Self::Target<'_> {
        Deref::deref(self)
    }
}

unsafe impl<T> FieldDeref for Option<&T> {
    type Target<'a> = Option<&'a T>
    where
        Self: 'a;

    #[inline]
    fn deref(&self) -> Self::Target<'_> {
        self.as_deref()
    }
}

unsafe impl<T> FieldDeref for Option<&mut T> {
    type Target<'a> = Option<&'a T>
    where
        Self: 'a;

    #[inline]
    fn deref(&self) -> Self::Target<'_> {
        self.as_deref()
    }
}

#[cfg(feature = "alloc")]
mod alloc_impl {
    use super::*;

    use alloc::boxed::Box;
    use alloc::rc::Rc;
    use alloc::sync::Arc;

    unsafe impl<T> FieldDeref for Box<T> {
        type Target<'a> = &'a T
        where
            Self: 'a;

        #[inline]
        fn deref(&self) -> Self::Target<'_> {
            Deref::deref(self)
        }
    }

    unsafe impl<T> FieldDeref for Rc<T> {
        type Target<'a> = &'a T
        where
            Self: 'a;

        #[inline]
        fn deref(&self) -> Self::Target<'_> {
            Deref::deref(self)
        }
    }

    unsafe impl<T> FieldDeref for Arc<T> {
        type Target<'a> = &'a T
        where
            Self: 'a;

        #[inline]
        fn deref(&self) -> Self::Target<'_> {
            Deref::deref(self)
        }
    }

    unsafe impl<T> FieldDeref for Option<Box<T>> {
        type Target<'a> = Option<&'a T>
        where
            Self: 'a;

        #[inline]
        fn deref(&self) -> Self::Target<'_> {
            self.as_deref()
        }
    }

    unsafe impl<T> FieldDeref for Option<Rc<T>> {
        type Target<'a> = Option<&'a T>
        where
            Self: 'a;

        #[inline]
        fn deref(&self) -> Self::Target<'_> {
            self.as_deref()
        }
    }

    unsafe impl<T> FieldDeref for Option<Arc<T>> {
        type Target<'a> = Option<&'a T>
        where
            Self: 'a;

        #[inline]
        fn deref(&self) -> Self::Target<'_> {
            self.as_deref()
        }
    }
}
