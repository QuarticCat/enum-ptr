pub trait PtrInfo {
    type Pointee: Sized; // ensure not fat pointer
    const ALIGN: usize = std::mem::align_of::<Self::Pointee>();
}

impl<T> PtrInfo for *const T {
    type Pointee = T;
}

impl<T> PtrInfo for *mut T {
    type Pointee = T;
}

impl<'a, T> PtrInfo for &'a T {
    type Pointee = T;
}

impl<'a, T> PtrInfo for &'a mut T {
    type Pointee = T;
}

impl<T> PtrInfo for Box<T> {
    type Pointee = T;
}

impl<'a, T> PtrInfo for Option<&'a T> {
    type Pointee = T;
}

impl<'a, T> PtrInfo for Option<&'a mut T> {
    type Pointee = T;
}

impl<T> PtrInfo for Option<Box<T>> {
    type Pointee = T;
}
