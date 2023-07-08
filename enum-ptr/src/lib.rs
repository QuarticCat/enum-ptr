//! # Basic Usage
//!
//! ```
//! use enum_ptr::{Aligned, Compact, EnumPtr, Unit};
//!
//! # #[derive(Debug, PartialEq, Eq, Clone)]
//! #[derive(EnumPtr)]
//! #[repr(C, usize)] // required
//! enum Foo<'a, T: Aligned> {
//!     A(T),       // supports any `T: Aligned`
//!     B(&'a u64),
//!     C(Unit),    // use `Unit` for unit variants
//! #    #[cfg(feature = "alloc")]
//!     D(Box<i64>),
//! }
//!
//! let compact_foo: Compact<_> = Foo::A(&0i32).into();
//! let original_foo: Foo<_> = compact_foo.into();
//! #
//! # let test = |f: Foo<&i32>| assert_eq!(f.clone(), Foo::from(Compact::from(f)));
//! # test(Foo::A(&0));
//! # test(Foo::B(&1));
//! # test(Foo::C(Unit::new()));
//! # #[cfg(feature = "alloc")]
//! # test(Foo::D(Box::new(2)));
//! ```
//!
//! You can implement [`Aligned`] for your own types.
//!
//! # Limitations
//!
//! Suppose we are deriving from `Foo`, then
//!
//! - **`Foo` must have a `#[repr(C, usize)]`.**
//!   - According to the [RFC] and the [Rust Reference], `#[repr(C, usize)]`
//!     guarantees the memory layout and discriminant values. Thus, we can
//!     safely transmute between two representations.
//! - **Each variant of `Foo` must have exactly one field.**
//!   - Unit variants are not allowed due to performance concerns.
//!   - If you need a unit variant, use [`Unit`].
//! - **Each variant of `Foo` must have enough alignment to store the tag.**
//!   - Currently this crate cannot utilize high bits.
//!
//! Any violation of these rules will either trigger a compilation error or
//! a run-time panic. Passed assertions will be optimized out. That is to say,
//! rule checks won't affect the run-time performance.
//!
//! [RFC]: https://github.com/rust-lang/rfcs/blob/master/text/2195-really-tagged-unions.md
//! [Rust Reference]: https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-fieldless-enumerations
//!
//! # Features
//!
//! - `alloc` *(default)* --- `Box`, `Rc`, and `Arc` support

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod compact;
mod compact_copy;
mod convert;
mod traits;
mod utils;

pub use compact::*;
pub use compact_copy::*;
pub use convert::*;
pub use traits::*;
pub use utils::*;

/// Derives conversions to and from [`Compact`].
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "alloc")] {
/// use enum_ptr::{EnumPtr, Unit};
///
/// #[derive(EnumPtr)]
/// #[enum_ptr(
///     // copy,    // derives conversions to and from `CompactCopy`
///     borrow(     // derives a reference type and `impl CompactBorrow`
///         name = "FooRef",
///     ),
///     borrow_mut( // derives a reference type and `impl CompactBorrowMut`
///         name = "FooRefMut",
///     ),
/// )]
/// #[repr(C, usize)]
/// enum Foo {
///     // unskipped fields must implement `FieldDeref` / `FieldDerefMut`
///     A(Box<i64>),         // ref type: `&i64` / `&mut i64`
///     B(Option<Box<i64>>), // ref type: `Option<&i64>` / `Option<&mut i64>`
///
///     // use `skip` to skip both, or use `skip_borrow` / `skip_borrow_mut`
///     #[enum_ptr(skip)]
///     C(Unit),             // ref type: `PhantomData` (skipped)
/// }
/// # }
/// ```
pub use enum_ptr_derive::EnumPtr;
