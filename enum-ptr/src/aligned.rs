use core::mem::align_of;

/// Mark that a type is properly aligned and can be used in `EnumPtr`.
///
/// # Safety
///
/// `T` must be aligned by `ALIGNMENT` (low bits are always zeros).
///
/// For example, raw pointers are not guaranteed to be aligned, so implementing
/// this trait for them is unsound.
pub unsafe trait Aligned {
    const ALIGNMENT: usize;
}

unsafe impl<'a, T> Aligned for &'a T {
    const ALIGNMENT: usize = align_of::<T>();
}

unsafe impl<'a, T> Aligned for &'a mut T {
    const ALIGNMENT: usize = align_of::<T>();
}

unsafe impl<'a, T> Aligned for Option<&'a T> {
    const ALIGNMENT: usize = align_of::<T>();
}

unsafe impl<'a, T> Aligned for Option<&'a mut T> {
    const ALIGNMENT: usize = align_of::<T>();
}

#[cfg(feature = "alloc")]
mod alloc_impl {
    use super::*;

    use alloc::boxed::Box;
    use alloc::rc::Rc;
    use alloc::sync::Arc;

    // TODO: remove it when `Ord::max` is made const
    const fn max(a: usize, b: usize) -> usize {
        if a > b {
            a
        } else {
            b
        }
    }

    unsafe impl<T> Aligned for Box<T> {
        const ALIGNMENT: usize = align_of::<T>();
    }

    /// It makes assumption on the layout of `Rc`. Use it with caution.
    unsafe impl<T> Aligned for Rc<T> {
        const ALIGNMENT: usize = max(align_of::<T>(), align_of::<usize>());
    }

    /// It makes assumption on the layout of `Arc`. Use it with caution.
    unsafe impl<T> Aligned for Arc<T> {
        const ALIGNMENT: usize = max(align_of::<T>(), align_of::<usize>());
    }

    unsafe impl<T> Aligned for Option<Box<T>> {
        const ALIGNMENT: usize = align_of::<T>();
    }

    /// It makes assumption on the layout of `Rc`. Use it with caution.
    unsafe impl<T> Aligned for Option<Rc<T>> {
        const ALIGNMENT: usize = max(align_of::<T>(), align_of::<usize>());
    }

    /// It makes assumption on the layout of `Arc`. Use it with caution.
    unsafe impl<T> Aligned for Option<Arc<T>> {
        const ALIGNMENT: usize = max(align_of::<T>(), align_of::<usize>());
    }
}
