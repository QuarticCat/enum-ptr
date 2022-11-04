use core::mem::transmute_copy;

#[repr(transparent)]
pub struct Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    _data: *const u8,
    phantom: core::marker::PhantomData<T>,
}

impl<T> Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    unsafe fn decompact_copy(&self) -> T {
        let this: Self = unsafe { transmute_copy(self) };
        T::from(this)
    }
}

impl<T> Drop for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    fn drop(&mut self) {
        let _ = unsafe { self.decompact_copy() };
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let this = unsafe { self.decompact_copy() };
        this.fmt(f)
    }
}

impl<T: Clone> Clone for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    fn clone(&self) -> Self {
        let this = unsafe { self.decompact_copy() };
        #[allow(clippy::redundant_clone)]
        this.clone().into()
    }
}

impl<T: PartialEq> PartialEq for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    fn eq(&self, other: &Self) -> bool {
        let this = unsafe { self.decompact_copy() };
        let that = unsafe { other.decompact_copy() };
        this.eq(&that)
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
        let this = unsafe { self.decompact_copy() };
        let that = unsafe { other.decompact_copy() };
        this.partial_cmp(&that)
    }
}

impl<T: Ord> Ord for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let this = unsafe { self.decompact_copy() };
        let that = unsafe { other.decompact_copy() };
        this.cmp(&that)
    }
}

impl<T: core::hash::Hash> core::hash::Hash for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let this = unsafe { self.decompact_copy() };
        this.hash(state);
    }
}
