use core::mem::align_of;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// Mark that a pointer type is properly aligned and can be used in `EnumPtr`.
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

#[cfg(feature = "alloc")]
unsafe impl<T> Aligned for Box<T> {
    const ALIGNMENT: usize = align_of::<T>();
}

unsafe impl<'a, T> Aligned for Option<&'a T> {
    const ALIGNMENT: usize = align_of::<T>();
}

unsafe impl<'a, T> Aligned for Option<&'a mut T> {
    const ALIGNMENT: usize = align_of::<T>();
}

#[cfg(feature = "alloc")]
unsafe impl<T> Aligned for Option<Box<T>> {
    const ALIGNMENT: usize = align_of::<T>();
}
