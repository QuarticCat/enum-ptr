use crate::{Compact, FieldDeref, FieldDerefMut};

#[doc(hidden)]
#[inline]
pub unsafe fn get_ref_helper<T, U>(
    compact: &Compact<T>,
    f: impl FnOnce(&T) -> Option<&U>,
) -> Option<<U as FieldDeref>::Target<'_>>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
    U: FieldDeref,
{
    compact.map_ref(|tmp| f(tmp).map(|tmp| tmp.force_deref()))
}

#[doc(hidden)]
#[inline]
pub unsafe fn get_mut_helper<T, U>(
    compact: &mut Compact<T>,
    f: impl FnOnce(&mut T) -> Option<&mut U>,
) -> Option<<U as FieldDerefMut>::Target<'_>>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
    U: FieldDerefMut,
{
    compact.map_mut(|tmp| f(tmp).map(|tmp| tmp.force_deref_mut()))
}

/// Borrows a variant from [`Compact`].
///
/// It requires the type of that variant implements [`FieldDeref`].
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "alloc")] {
/// use enum_ptr::{get_ref, Compact, EnumPtr};
///
/// #[derive(EnumPtr)]
/// #[repr(C, usize)]
/// enum Foo {
///     A(Box<i32>),
///     B(Box<u32>),
/// }
///
/// let foo: Compact<_> = Foo::A(Box::new(1)).into();
/// assert_eq!(get_ref!(foo, Foo::A), Some(&1));
/// assert_eq!(get_ref!(foo, Foo::B), None);
/// # }
/// ```
#[macro_export]
macro_rules! get_ref {
    ($compact:expr, $variant:path) => {
        unsafe {
            $crate::get_ref_helper(&$compact, |tmp| match tmp {
                $variant(inner) => Some(inner),
                _ => None,
            })
        }
    };
}

/// Mutably borrows a variant from [`Compact`].
///
/// It requires the type of that variant implements [`FieldDerefMut`].
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "alloc")] {
/// use enum_ptr::{get_mut, Compact, EnumPtr};
///
/// #[derive(EnumPtr)]
/// #[repr(C, usize)]
/// enum Foo {
///     A(Box<i32>),
///     B(Box<u32>),
/// }
///
/// let mut foo: Compact<_> = Foo::A(Box::new(1)).into();
/// assert_eq!(get_mut!(foo, Foo::A), Some(&mut 1));
/// assert_eq!(get_mut!(foo, Foo::B), None);
/// # }
/// ```
#[macro_export]
macro_rules! get_mut {
    ($compact:expr, $variant:path) => {
        unsafe {
            $crate::get_mut_helper(&mut $compact, |tmp| match tmp {
                $variant(inner) => Some(inner),
                _ => None,
            })
        }
    };
}
