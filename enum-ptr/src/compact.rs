use core::mem::{transmute_copy, ManuallyDrop};

// TODO: impl as many traits as possible

/// The compact representation of `T`. Only one pointer wide.
#[repr(transparent)]
pub struct Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    data: *const u8,
    phantom: core::marker::PhantomData<T>,
}

impl<T> Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    /// Get the inner data.
    #[inline]
    pub fn inner(&self) -> *const u8 {
        self.data
    }

    /// An alias of `T::from(self)`.
    #[inline]
    pub fn extract(self) -> T {
        self.into()
    }

    #[inline]
    unsafe fn extract_copy(&self) -> T {
        let this: Self = unsafe { transmute_copy(self) };
        T::from(this)
    }

    /// # Examples
    ///
    /// ```
    /// # use enum_ptr::{Compact, EnumPtr};
    /// #
    /// # #[derive(EnumPtr, Debug)]
    /// # #[repr(C, usize)]
    /// # enum Foo<'a, 'b> {
    /// #     A(&'a i32),
    /// #     B(&'b i32),
    /// # }
    /// #
    /// let mut foo: Compact<_> = Foo::A(&1).into();
    /// foo.map_ref(|f: &Foo| println!("{f:?}"));
    /// ```
    #[inline]
    pub fn map_ref<U>(&self, func: impl FnOnce(&T) -> U) -> U {
        let this = unsafe { ManuallyDrop::new(self.extract_copy()) };
        func(&this)
    }

    /// # Examples
    ///
    /// ```
    /// # use enum_ptr::{Compact, EnumPtr};
    /// #
    /// # #[derive(EnumPtr, Debug)]
    /// # #[repr(C, usize)]
    /// # enum Foo<'a, 'b> {
    /// #     A(&'a i32),
    /// #     B(&'b i32),
    /// # }
    /// #
    /// let mut foo: Compact<_> = Foo::A(&1).into();
    /// foo.map_ref_mut(|f: &mut Foo| println!("{f:?}"));
    /// ```
    #[inline]
    pub fn map_ref_mut<U>(&mut self, func: impl FnOnce(&mut T) -> U) -> U {
        let mut this = unsafe { ManuallyDrop::new(self.extract_copy()) };
        func(&mut this)
    }
}

impl<T> Compact<T>
where
    T: From<Compact<T>> + Clone,
    Compact<T>: From<T>,
{
    /// An alias of `self.map_ref(|t| t.clone())`.
    #[inline]
    pub fn extract_clone(&self) -> T {
        self.map_ref(|this| this.clone())
    }
}

impl<T> Drop for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    #[inline]
    fn drop(&mut self) {
        let _ = unsafe { self.extract_copy() };
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.map_ref(|this| this.fmt(f))
    }
}

impl<T: Clone> Clone for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    #[inline]
    fn clone(&self) -> Self {
        self.map_ref(|this| this.clone().into())
    }
}

// TODO: find a way to mark `Copy` when `T: Copy`

impl<T: PartialEq> PartialEq for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.map_ref(|this| other.map_ref(|that| this.eq(that)))
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
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.map_ref(|this| other.map_ref(|that| this.partial_cmp(that)))
    }
}

impl<T: Ord> Ord for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.map_ref(|this| other.map_ref(|that| this.cmp(that)))
    }
}

impl<T: core::hash::Hash> core::hash::Hash for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.map_ref(|this| this.hash(state))
    }
}

unsafe impl<T> Send for Compact<T>
where
    T: From<Compact<T>> + Send,
    Compact<T>: From<T>,
{
}

unsafe impl<T> Sync for Compact<T>
where
    T: From<Compact<T>> + Sync,
    Compact<T>: From<T>,
{
}
