use core::marker::PhantomData;
use core::mem::ManuallyDrop;

use crate::{CompactBorrow, CompactBorrowMut};

/// Compact representation of `T`. Only one-pointer wide.
///
/// It behaves like `T` for `Drop`, `Clone`, `Hash`, `Eq`, `Ord`, ...
#[repr(transparent)]
pub struct Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    data: *const u8,
    marker: PhantomData<T>,
}

impl<T> Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    /// Returns the underlying raw data.
    #[inline]
    pub fn as_raw_data(&self) -> *const u8 {
        self.data
    }

    /// Alias of `T::from(self)`.
    #[inline]
    pub fn extract(self) -> T {
        self.into()
    }

    /// Maps a `&T` to `U` by applying a function to a temporarily created
    /// `T` value.
    ///
    /// Since the value is temporary, you cannot take references to it out
    /// from this function.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "alloc")] {
    /// use enum_ptr::{Compact, EnumPtr};
    ///
    /// #[derive(EnumPtr, Debug)]
    /// #[repr(C, usize)]
    /// enum Foo {
    ///     A(Box<i32>),
    ///     B(Box<u32>),
    /// }
    ///
    /// let mut foo: Compact<_> = Foo::A(Box::new(1)).into();
    /// let result = foo.map_ref(|f| match f {
    ///     Foo::A(r) => **r,
    ///     _ => unreachable!(),
    /// });
    /// assert_eq!(result, 1);
    /// # }
    /// ```
    #[inline]
    pub fn map_ref<U>(&self, f: impl FnOnce(&T) -> U) -> U {
        f(&ManuallyDrop::new(T::from(Self { ..*self })))
    }

    /// Maps a `&mut T` to `U` by applying a function to a temporarily created
    /// `T` value.
    ///
    /// Since the value is temporary, you cannot take references to it out
    /// from this function.
    ///
    /// # Safety
    ///
    /// See issue [#3](https://github.com/QuarticCat/enum-ptr/issues/3).
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "alloc")] {
    /// use enum_ptr::{Compact, EnumPtr};
    ///
    /// #[derive(EnumPtr, Debug, PartialEq, Eq)]
    /// #[repr(C, usize)]
    /// enum Foo {
    ///     A(Box<i32>),
    ///     B(Box<u32>),
    /// }
    ///
    /// let mut foo: Compact<_> = Foo::A(Box::new(1)).into();
    /// unsafe {
    ///     foo.map_mut(|f| match f {
    ///         Foo::A(r) => **r = 2,
    ///         _ => unreachable!(),
    ///     });
    /// }
    /// assert_eq!(foo.extract(), Foo::A(Box::new(2)));
    /// # }
    /// ```
    #[inline]
    pub unsafe fn map_mut<U>(&mut self, f: impl FnOnce(&mut T) -> U) -> U {
        f(&mut ManuallyDrop::new(T::from(Self { ..*self })))
    }

    // /// Replaces the wrapped value with a new one computed from f, returning
    // /// the old value, without deinitializing either one.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// # #[cfg(feature = "alloc")] {
    // /// use enum_ptr::{Compact, EnumPtr};
    // ///
    // /// #[derive(EnumPtr, Debug, PartialEq, Eq)]
    // /// #[repr(C, usize)]
    // /// enum Foo {
    // ///     A(Box<i32>),
    // ///     B(Box<u32>),
    // /// }
    // ///
    // /// let mut foo: Compact<_> = Foo::A(Box::new(1)).into();
    // /// let old = foo.replace_with(|_| Foo::B(Box::new(2)));
    // /// assert_eq!(old, Foo::A(Box::new(1)));
    // /// assert_eq!(foo.extract(), Foo::B(Box::new(2)));
    // /// # }
    // /// ```
    // #[inline]
    // pub fn replace_with(&mut self, f: impl FnOnce(&mut T) -> T) -> T {
    //     let mut old = T::from(Self { ..*self });
    //     let new = f(&mut old);
    //     self.data = unsafe { crate::compact(new) };
    //     old
    // }
}

impl<T> Compact<T>
where
    T: From<Compact<T>> + CompactBorrow,
    Compact<T>: From<T>,
{
    #[inline]
    pub fn borrow(&self) -> <T as CompactBorrow>::Target<'_> {
        CompactBorrow::borrow(self)
    }
}

impl<T> Compact<T>
where
    T: From<Compact<T>> + CompactBorrowMut,
    Compact<T>: From<T>,
{
    #[inline]
    pub fn borrow_mut(&mut self) -> <T as CompactBorrowMut>::Target<'_> {
        CompactBorrowMut::borrow_mut(self)
    }
}

impl<T> Drop for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
{
    #[inline]
    fn drop(&mut self) {
        drop(T::from(Self { ..*self }));
    }
}

impl<T> Clone for Compact<T>
where
    T: From<Compact<T>> + Clone,
    Compact<T>: From<T>,
{
    #[inline]
    fn clone(&self) -> Self {
        self.map_ref(|this| this.clone().into())
    }
}

impl<T> PartialEq for Compact<T>
where
    T: From<Compact<T>> + PartialEq,
    Compact<T>: From<T>,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.map_ref(|this| other.map_ref(|that| this.eq(that)))
    }
}

impl<T> Eq for Compact<T>
where
    T: From<Compact<T>> + Eq,
    Compact<T>: From<T>,
{
}

impl<T> PartialOrd for Compact<T>
where
    T: From<Compact<T>> + PartialOrd,
    Compact<T>: From<T>,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.map_ref(|this| other.map_ref(|that| this.partial_cmp(that)))
    }
}

impl<T> Ord for Compact<T>
where
    T: From<Compact<T>> + Ord,
    Compact<T>: From<T>,
{
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.map_ref(|this| other.map_ref(|that| this.cmp(that)))
    }
}

impl<T> core::fmt::Debug for Compact<T>
where
    T: From<Compact<T>> + core::fmt::Debug,
    Compact<T>: From<T>,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.map_ref(|this| this.fmt(f))
    }
}

impl<T> Default for Compact<T>
where
    T: From<Compact<T>> + Default,
    Compact<T>: From<T>,
{
    fn default() -> Self {
        T::default().into()
    }
}

impl<T> core::hash::Hash for Compact<T>
where
    T: From<Compact<T>> + core::hash::Hash,
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
