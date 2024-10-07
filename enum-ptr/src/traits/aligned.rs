use core::mem::align_of;

/// Types (may not be pointers) that can be used in [`EnumPtr`](crate::EnumPtr).
///
/// # Safety
///
/// - `T` must be exactly one-pointer wide.
/// - `T`'s pointee must be aligned by `ALIGNMENT` (`T`'s low bits are zeros).
///
/// For example, raw pointers are not guaranteed to be aligned, so implementing
/// this trait for them is unsound.
///
/// # Examples
///
/// ```
/// use enum_ptr::{Aligned, Compact, EnumPtr};
///
/// // It's your responsibility to ensure `MyPtr` is always aligned.
/// struct MyPtr<T>(*const T);
///
/// unsafe impl<T> Aligned for MyPtr<T> {
///     const ALIGNMENT: usize = std::mem::align_of::<T>();
/// }
///
/// #[derive(EnumPtr)]
/// #[repr(C, usize)]
/// enum Foo {
///     A(MyPtr<i64>),
///     B(MyPtr<u64>),
/// }
/// ```
pub unsafe trait Aligned {
    const ALIGNMENT: usize;
}

unsafe impl<T> Aligned for &T {
    const ALIGNMENT: usize = align_of::<T>();
}

unsafe impl<T> Aligned for &mut T {
    const ALIGNMENT: usize = align_of::<T>();
}

unsafe impl<T> Aligned for Option<&T> {
    const ALIGNMENT: usize = align_of::<T>();
}

unsafe impl<T> Aligned for Option<&mut T> {
    const ALIGNMENT: usize = align_of::<T>();
}

#[cfg(feature = "alloc")]
mod alloc_impl {
    use super::*;

    use alloc::boxed::Box;
    use alloc::rc::Rc;
    use alloc::sync::Arc;

    // TODO: remove it when `Ord::max` is made const
    const fn max(a: usize, b: usize) -> usize {
        if a > b {
            a
        } else {
            b
        }
    }

    unsafe impl<T> Aligned for Box<T> {
        const ALIGNMENT: usize = align_of::<T>();
    }

    /// Implementing [`Aligned`] for [`Rc`] relies on some undocumented
    /// assumptions on its internal representation. However, it's very unlikely
    /// for these assumptions to be violated in practice.
    /// [`RcBox` is marked as `repr(C)` and reference counters are typed as `usize`][1].
    /// Thus, its alignment is defined as [the highest of its members][2] (i.e. among
    /// max of `usize` and `T`). Lastly, it's unconceivable for the type to be
    /// lifted with `repr(C)` as commented or to start to use shorter
    /// integers as counters.
    ///
    /// [1]: https://doc.rust-lang.org/1.81.0/src/alloc/rc.rs.html#286-291
    /// [2]: https://doc.rust-lang.org/reference/type-layout.html#reprc-structs
    unsafe impl<T> Aligned for Rc<T> {
        const ALIGNMENT: usize = max(align_of::<T>(), align_of::<usize>());
    }

    /// Implementing [`Aligned`] for [`Arc`] relies on some undocumented
    /// assumptions on its internal representation. However, it's very unlikely
    /// for these assumptions to be violated in practice.
    /// [`ArcInner` is marked as `repr(C)` and reference counters are typed as `usize`][1].
    /// Thus, its alignment is defined as [the highest of its members][2] (i.e. among
    /// max of `usize` and `T`). Lastly, it's unconceivable for the type to be
    /// lifted with `repr(C)` as commented or to start to use shorter
    /// integers as counters.
    ///
    /// [1]: https://doc.rust-lang.org/1.81.0/src/alloc/sync.rs.html#349-359
    /// [2]: https://doc.rust-lang.org/reference/type-layout.html#reprc-structs
    unsafe impl<T> Aligned for Arc<T> {
        const ALIGNMENT: usize = max(align_of::<T>(), align_of::<usize>());
    }

    unsafe impl<T> Aligned for Option<Box<T>> {
        const ALIGNMENT: usize = align_of::<T>();
    }

    /// On top of the safety reasoning of `impl Align for Rc<T>`, implementing [`Aligned`]
    /// for `Option<Rc<T>>` is safe as well. `Rc` holds [a pointer to its `RcBox` as
    /// `NonNull<RcBox<T>>`][1]
    /// And the `Option`-ed type (i.e. `Option<NonNull<RcBox<T>>>`) is explicitly guaranteed
    /// to be [same as the original type in terms of size and alignment][2].
    ///
    /// [1]: https://doc.rust-lang.org/1.81.0/src/alloc/rc.rs.html#315-322
    /// [2]: https://doc.rust-lang.org/nightly/std/option/index.html#representation
    unsafe impl<T> Aligned for Option<Rc<T>> {
        const ALIGNMENT: usize = max(align_of::<T>(), align_of::<usize>());
    }

    /// On top of the safety reasoning of `impl Align for Arc<T>`, implementing [`Aligned`]
    /// for `Option<Arc<T>>` is safe as well. `Arc` holds [a pointer to its `ArcInner` as
    /// `NonNull<ArcInner<T>>`][1]
    /// And the `Option`-ed type (i.e. `Option<NonNull<ArcInner<T>>>`) is explicitly guaranteed
    /// to be [same as the original type in terms of size and alignment][2].
    ///
    /// [1]: https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#241-248
    /// [2]: https://doc.rust-lang.org/nightly/std/option/index.html#representation
    unsafe impl<T> Aligned for Option<Arc<T>> {
        const ALIGNMENT: usize = max(align_of::<T>(), align_of::<usize>());
    }
}
