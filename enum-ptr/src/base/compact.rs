use core::mem::{transmute_copy, ManuallyDrop};

use crate::{CompactBorrow, CompactBorrowMut, CompactInnerCopy, Compactable};

/// Compact representation of `T`. Only one-pointer wide.
///
/// It behaves like `T` for `Drop`, `Clone`, `Hash`, `Eq`, `Ord`, ...
#[repr(transparent)]
pub struct Compact<T: Compactable> {
    pub(crate) inner: T::Inner,
}

impl<T: Compactable> Compact<T> {
    /// Returns the underlying raw data.
    #[inline]
    pub fn as_raw_data(&self) -> *const u8 {
        unsafe { transmute_copy(self) }
    }

    /// Returns the original value.
    #[inline]
    pub fn extract(self) -> T {
        T::extract(self)
    }

    #[inline]
    unsafe fn temp_extract(&self) -> ManuallyDrop<T> {
        ManuallyDrop::new(T::extract(transmute_copy(self)))
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
        unsafe { f(&self.temp_extract()) }
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
        f(&mut self.temp_extract())
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

impl<T: CompactBorrow> Compact<T> {
    /// Returns a reference type that acts like `&T`.
    ///
    /// Check [`EnumPtr`](crate::EnumPtr) for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "alloc")] {
    /// use enum_ptr::{Compact, EnumPtr};
    ///
    /// #[derive(EnumPtr, Debug)]
    /// #[enum_ptr(borrow)] // required
    /// #[repr(C, usize)]
    /// enum Foo {               // enum FooRef<'enum_ptr> {
    ///     A(Box<i32>),         //     A(&'enum_ptr i32),
    ///     B(Option<Box<u32>>), //     B(Option<&'enum_ptr u32>),
    /// }                        // }
    ///
    /// let foo: Compact<_> = Foo::A(Box::new(1)).into();
    /// match foo.borrow() {
    ///     FooRef::A(inner) => assert_eq!(inner, &1),
    ///     _ => unreachable!(),
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn borrow(&self) -> <T as CompactBorrow>::Target<'_> {
        T::borrow(self)
    }
}

impl<T: CompactBorrowMut> Compact<T> {
    /// Returns a reference type that acts like `&mut T`.
    ///
    /// Check [`EnumPtr`](crate::EnumPtr) for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "alloc")] {
    /// use enum_ptr::{Compact, EnumPtr};
    ///
    /// #[derive(EnumPtr, Debug)]
    /// #[enum_ptr(borrow_mut)] // required
    /// #[repr(C, usize)]
    /// enum Foo {               // enum FooRefMut<'enum_ptr> {
    ///     A(Box<i32>),         //     A(&'enum_ptr mut i32),
    ///     B(Option<Box<u32>>), //     B(Option<&'enum_ptr mut u32>),
    /// }                        // }
    ///
    /// let mut foo: Compact<_> = Foo::A(Box::new(1)).into();
    /// match foo.borrow_mut() {
    ///     FooRefMut::A(inner) => assert_eq!(inner, &mut 1),
    ///     _ => unreachable!(),
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn borrow_mut(&mut self) -> <T as CompactBorrowMut>::Target<'_> {
        T::borrow_mut(self)
    }
}

impl<T: Compactable> Clone for Compact<T>
where
    T::Inner: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        let inner = self.inner.clone();
        Self { inner }
    }
}

impl<T: Compactable<Inner = CompactInnerCopy<T>> + Copy> Copy for Compact<T> {}

impl<T: Compactable + PartialEq> PartialEq for Compact<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.map_ref(|this| other.map_ref(|that| this.eq(that)))
    }
}

impl<T: Compactable + Eq> Eq for Compact<T> {}

impl<T: Compactable + PartialOrd> PartialOrd for Compact<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.map_ref(|this| other.map_ref(|that| this.partial_cmp(that)))
    }
}

impl<T: Compactable + Ord> Ord for Compact<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.map_ref(|this| other.map_ref(|that| this.cmp(that)))
    }
}

impl<T: Compactable + core::fmt::Debug> core::fmt::Debug for Compact<T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.map_ref(|this| this.fmt(f))
    }
}

impl<T: Compactable + Default> Default for Compact<T> {
    fn default() -> Self {
        T::default().compact()
    }
}

impl<T: Compactable + core::hash::Hash> core::hash::Hash for Compact<T> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.map_ref(|this| this.hash(state))
    }
}

unsafe impl<T: Compactable + Send> Send for Compact<T> {}

unsafe impl<T: Compactable + Sync> Sync for Compact<T> {}
