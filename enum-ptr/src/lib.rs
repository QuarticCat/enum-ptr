//! # Basic Usage
//!
//! ```
//! use enum_ptr::{Aligned, Compact, EnumPtr, Unit};
//!
//! # #[derive(Debug, PartialEq, Eq, Clone)]
//! #[derive(EnumPtr)]
//! #[repr(C, usize)] // required
//! enum Foo<'a> {
//!     A(&'a u64),
//! #    #[cfg(feature = "alloc")]
//!     C(Option<Box<i64>>),
//!     D(Unit),             // use `Unit` for unit variants
//! }
//!
//! let compact_foo: Compact<_> = Foo::A(&0).into();
//! let original_foo: Foo = compact_foo.into();
//! #
//! # let test = |f: Foo| assert_eq!(Foo::from(Compact::from(f.clone())), f);
//! # test(Foo::A(&0));
//! # #[cfg(feature = "alloc")]
//! # test(Foo::C(Some(Box::new(2))));
//! # test(Foo::D(enum_ptr::UNIT));
//! ```
//!
//! # Extension
//!
//! You can implement [`Aligned`] for your own types (may not be pointers).
//!
//! ```
//! use enum_ptr::{Aligned, Compact, EnumPtr};
//!
//! struct MyPtr<T>(*const T);
//!
//! unsafe impl<T> Aligned for MyPtr<T> {
//!     const ALIGNMENT: usize = std::mem::align_of::<T>();
//! }
//!
//! #[derive(EnumPtr)]
//! #[repr(C, usize)]
//! enum Foo {
//!     A(MyPtr<i64>),
//!     B(MyPtr<u64>),
//! }
//! ```
//!
//! # Limitations
//!
//! Suppose we are deriving from `Foo`, then
//!
//! - **`Foo` must be 2 pointers wide.**
//!   - If `Foo` is smaller, it is already in the compact representation.
//!   - If `Foo` is larger, this crate cannot compress it into a `usize`.
//! - **`Foo` must have a `#[repr(C, usize)]`.**
//!   - According to the [RFC] and the [Rust Reference], `#[repr(C, usize)]`
//!     guarantees the memory layout and discriminant values. Thus, we can
//!     safely transmute between two representations.
//! - **Each variant of `Foo` must have exactly one field.**
//!   - Unit variants are not allowed due to performance concerns.
//!   - If you need a unit variant, use [`Unit`].
//! - **Each variant of `Foo` must have enough alignment to store the tag.**
//!
//! Any violation of these rules will trigger a compilation error except
//! the alignment rule. If not, please file an issue.
//!
//! If some variant has no enough alignment, it will trigger a run-time panic.
//! Otherwise, assertions will be optimized away.
//!
//! [RFC]: https://github.com/rust-lang/rfcs/blob/master/text/2195-really-tagged-unions.md
//! [Rust Reference]: https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-fieldless-enumerations
//!
//! # Features
//!
//! - `alloc` (default) - `Box`, `Rc`, and `Arc` support

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod aligned;
mod compact;
mod utils;

pub use aligned::*;
pub use compact::*;
pub use utils::*;

pub use enum_ptr_derive::EnumPtr;
