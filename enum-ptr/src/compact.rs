use core::mem::transmute_copy;

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
        let this: Self = unsafe { transmute_copy(self) };
        let _ = T::from(this);
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let this: Self = unsafe { transmute_copy(self) };
        T::from(this).fmt(f)
    }
}

impl<T: Clone> Clone for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    fn clone(&self) -> Self {
        let this: Self = unsafe { transmute_copy(self) };
        #[allow(clippy::redundant_clone)] // intended
        T::from(this).clone().into()
    }
}

impl<T: PartialEq> PartialEq for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    fn eq(&self, other: &Self) -> bool {
        let this: Self = unsafe { transmute_copy(self) };
        let that: Self = unsafe { transmute_copy(other) };
        T::from(this).eq(&T::from(that))
    }
}

impl<T: Eq> Eq for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
}

impl<T: PartialOrd> PartialOrd for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        let this: Self = unsafe { transmute_copy(self) };
        let that: Self = unsafe { transmute_copy(other) };
        T::from(this).partial_cmp(&T::from(that))
    }
}

impl<T: Ord> Ord for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let this: Self = unsafe { transmute_copy(self) };
        let that: Self = unsafe { transmute_copy(other) };
        T::from(this).cmp(&T::from(that))
    }
}

impl<T: core::hash::Hash> core::hash::Hash for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let this: Self = unsafe { transmute_copy(self) };
        T::from(this).hash(state);
    }
}
