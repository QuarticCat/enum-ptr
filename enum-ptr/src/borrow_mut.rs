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

/// Types that can be used to derive [`CompactBorrowMut`].
///
/// It's like [`DerefMut`] but with flexible targets and strict constraints.
///
/// # Safety
///
/// `T` must not `deref_mut` to something that points to its own memory.
///
/// A counter-example is `ManuallyDrop<T>`, which will `deref_mut` to `&mut T`.
pub unsafe trait FieldDerefMut {
    type Target<'a>
    where
        Self: 'a;

    fn deref_mut(&mut self) -> Self::Target<'_>;
}

unsafe impl<T> FieldDerefMut for &mut T {
    type Target<'a> = &'a mut T
    where
        Self: 'a;

    fn deref_mut(&mut self) -> Self::Target<'_> {
        DerefMut::deref_mut(self)
    }
}

unsafe impl<T> FieldDerefMut for Option<&mut T> {
    type Target<'a> = Option<&'a mut T>
    where
        Self: 'a;

    fn deref_mut(&mut self) -> Self::Target<'_> {
        self.as_deref_mut()
    }
}

#[cfg(feature = "alloc")]
mod alloc_impl {
    use super::*;

    use alloc::boxed::Box;

    unsafe impl<T> FieldDerefMut for Box<T> {
        type Target<'a> = &'a mut T
        where
            Self: 'a;

        fn deref_mut(&mut self) -> Self::Target<'_> {
            DerefMut::deref_mut(self)
        }
    }

    unsafe impl<T> FieldDerefMut for Option<Box<T>> {
        type Target<'a> = Option<&'a mut T>
        where
            Self: 'a;

        fn deref_mut(&mut self) -> Self::Target<'_> {
            self.as_deref_mut()
        }
    }
}
