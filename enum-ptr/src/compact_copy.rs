use core::marker::PhantomData;

/// A `Copy` version of [`Compact`](crate::Compact).
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CompactCopy<T>
where
    T: From<CompactCopy<T>> + Copy,
    CompactCopy<T>: From<T>,
{
    data: *const u8,
    marker: PhantomData<T>,
}

impl<T> CompactCopy<T>
where
    T: From<CompactCopy<T>> + Copy,
    CompactCopy<T>: From<T>,
{
    /// Get the inner data.
    #[inline]
    pub fn inner(&self) -> *const u8 {
        self.data
    }

    /// Alias of `T::from(self)`.
    #[inline]
    pub fn extract(self) -> T {
        self.into()
    }
}

impl<T> PartialEq for CompactCopy<T>
where
    T: From<CompactCopy<T>> + Copy + PartialEq,
    CompactCopy<T>: From<T>,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.extract().eq(&other.extract())
    }
}

impl<T> Eq for CompactCopy<T>
where
    T: From<CompactCopy<T>> + Copy + Eq,
    CompactCopy<T>: From<T>,
{
}

impl<T> PartialOrd for CompactCopy<T>
where
    T: From<CompactCopy<T>> + Copy + PartialOrd,
    CompactCopy<T>: From<T>,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.extract().partial_cmp(&other.extract())
    }
}

impl<T> Ord for CompactCopy<T>
where
    T: From<CompactCopy<T>> + Copy + Ord,
    CompactCopy<T>: From<T>,
{
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.extract().cmp(&other.extract())
    }
}

impl<T> core::fmt::Debug for CompactCopy<T>
where
    T: From<CompactCopy<T>> + Copy + core::fmt::Debug,
    CompactCopy<T>: From<T>,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.extract().fmt(f)
    }
}

impl<T> Default for CompactCopy<T>
where
    T: From<CompactCopy<T>> + Copy + Default,
    CompactCopy<T>: From<T>,
{
    fn default() -> Self {
        T::default().into()
    }
}

impl<T> core::hash::Hash for CompactCopy<T>
where
    T: From<CompactCopy<T>> + Copy + core::hash::Hash,
    CompactCopy<T>: From<T>,
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.extract().hash(state)
    }
}

unsafe impl<T> Send for CompactCopy<T>
where
    T: From<CompactCopy<T>> + Copy + Send,
    CompactCopy<T>: From<T>,
{
}

unsafe impl<T> Sync for CompactCopy<T>
where
    T: From<CompactCopy<T>> + Copy + Sync,
    CompactCopy<T>: From<T>,
{
}
