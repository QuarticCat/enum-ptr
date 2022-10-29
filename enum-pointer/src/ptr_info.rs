pub trait PtrInfo {
    type Pointee;
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
