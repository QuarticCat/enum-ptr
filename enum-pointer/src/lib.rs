#![cfg_attr(not(feature = "std"), no_std)]

pub use enum_pointer_derive::EnumPointer;

/// # Safety
///
/// - `T` is aligned by `align_of::<Pointee>()` (low bits are always zeros)
/// - `size_of::<T>() == size_of::<usize>()`
pub unsafe trait Compactable: Sized {
    type Pointee;
    const ALIGN: usize = core::mem::align_of::<Self::Pointee>();
}

unsafe impl<T> Compactable for *const T {
    type Pointee = T;
}

unsafe impl<T> Compactable for *mut T {
    type Pointee = T;
}

unsafe impl<'a, T> Compactable for &'a T {
    type Pointee = T;
}

unsafe impl<'a, T> Compactable for &'a mut T {
    type Pointee = T;
}

#[cfg(feature = "std")]
unsafe impl<T> Compactable for Box<T> {
    type Pointee = T;
}

unsafe impl<'a, T> Compactable for Option<&'a T> {
    type Pointee = T;
}

unsafe impl<'a, T> Compactable for Option<&'a mut T> {
    type Pointee = T;
}

#[cfg(feature = "std")]
unsafe impl<T> Compactable for Option<Box<T>> {
    type Pointee = T;
}
