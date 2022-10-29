pub use enum_pointer_derive::EnumPointer;

pub trait Compactable: Sized {
    type Pointee;
    const ALIGN: usize = std::mem::align_of::<Self::Pointee>();
}

impl<T> Compactable for *const T {
    type Pointee = T;
}

impl<T> Compactable for *mut T {
    type Pointee = T;
}

impl<'a, T> Compactable for &'a T {
    type Pointee = T;
}

impl<'a, T> Compactable for &'a mut T {
    type Pointee = T;
}

impl<T> Compactable for Box<T> {
    type Pointee = T;
}

impl<'a, T> Compactable for Option<&'a T> {
    type Pointee = T;
}

impl<'a, T> Compactable for Option<&'a mut T> {
    type Pointee = T;
}

impl<T> Compactable for Option<Box<T>> {
    type Pointee = T;
}
