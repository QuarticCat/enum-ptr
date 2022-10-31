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
