#![cfg_attr(not(feature = "std"), no_std)]

pub use enum_ptr_derive::EnumPtr;

/// # Safety
///
/// - `T` is aligned by `align_of::<Pointee>()` (low bits are always zeros)
pub unsafe trait Compactable: Sized {
    type Pointee;
    const ALIGN: usize = core::mem::align_of::<Self::Pointee>();
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

#[repr(transparent)]
pub struct Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    _data: usize,
    phantom: core::marker::PhantomData<T>,
}

impl<T> Drop for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    fn drop(&mut self) {
        let this: Self = unsafe { ::core::mem::transmute_copy(self) };
        let _ = T::from(this);
    }
}
