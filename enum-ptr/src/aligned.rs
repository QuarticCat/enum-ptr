/// # Safety
///
/// - `T` is aligned by `align_of::<Pointee>()` (low bits are always zeros)
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
