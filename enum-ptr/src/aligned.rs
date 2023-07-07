use core::mem::align_of;

/// Types (may not be pointers) that can be used in `EnumPtr`.
///
/// # Safety
///
/// - `T` must be exactly one-pointer wide.
/// - `T`'s pointee must be aligned by `ALIGNMENT` (`T`'s low bits are zeros).
///
/// For example, raw pointers are not guaranteed to be aligned, so implementing
/// this trait for them is unsound.
///
/// # Examples
///
/// ```
/// use enum_ptr::{Aligned, Compact, EnumPtr};
///
/// // It's your responsibility to ensure `MyPtr` is always aligned.
/// struct MyPtr<T>(*const T);
///
/// unsafe impl<T> Aligned for MyPtr<T> {
///     const ALIGNMENT: usize = std::mem::align_of::<T>();
/// }
///
/// #[derive(EnumPtr)]
/// #[repr(C, usize)]
/// enum Foo {
///     A(MyPtr<i64>),
///     B(MyPtr<u64>),
/// }
/// ```
pub unsafe trait Aligned {
    const ALIGNMENT: usize;
}

unsafe impl<T> Aligned for &T {
    const ALIGNMENT: usize = align_of::<T>();
}

unsafe impl<T> Aligned for &mut T {
    const ALIGNMENT: usize = align_of::<T>();
}

unsafe impl<T> Aligned for Option<&T> {
    const ALIGNMENT: usize = align_of::<T>();
}

unsafe impl<T> Aligned for Option<&mut T> {
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
