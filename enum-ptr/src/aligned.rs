/// # Safety
///
/// `T` must be aligned by `align_of::<Pointee>()` (low bits are always zeros).
///
/// For example, raw pointers are not guaranteed to be aligned, so implementing
/// this trait for them is unsound.
pub unsafe trait Aligned {
    type Pointee;
}

unsafe impl<'a, T> Aligned for &'a T {
    type Pointee = T;
}

unsafe impl<'a, T> Aligned for &'a mut T {
    type Pointee = T;
}

#[cfg(feature = "std")]
unsafe impl<T> Aligned for Box<T> {
    type Pointee = T;
}

unsafe impl<'a, T> Aligned for Option<&'a T> {
    type Pointee = T;
}

unsafe impl<'a, T> Aligned for Option<&'a mut T> {
    type Pointee = T;
}

#[cfg(feature = "std")]
unsafe impl<T> Aligned for Option<Box<T>> {
    type Pointee = T;
}
