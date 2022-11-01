//! # Basic Usage
//!
//! ```
//! use enum_ptr::{Compact, EnumPtr};
//!
//! #[derive(EnumPtr)]
//! #[repr(C, usize)] // required
//! enum Foo<'a, 'b, T> {
//!     A(&'a T),
//!     B { ptr: &'b mut i64 },
//!     C(Option<Box<i64>>),
//!     D(),
//!     E {},
//!     F,
//! }
//!
//! let compact_foo: Compact<_> = Foo::A(&0u64).into();
//! let original_foo: Foo<_> = compact_foo.into();
//! ```
//!
//! - The `enum` can have generic parameters.
//! - Its variants can be named (`X{...}`), unnamed (`X(...)`), or units (`X`).
//! - Each variant can have at most one field.
//! - Fields are required to implement the [`Aligned`] trait.
//!
//! # Extension
//!
//! To use your own pointer types in the fields, you only need to implement
//! the [`Aligned`] trait for it. Note that you are responsible to ensure the
//! safety assertions of [`Aligned`].
//!
//! ```
//! use enum_ptr::{Aligned, Compact, EnumPtr};
//!
//! struct MyPtr<T>(*const T);
//!
//! unsafe impl<T> Aligned for MyPtr<T> {
//!     type Pointee = T;
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
//! # Caveats
//!
//! Due to safety issues, when the `enum` contains units, the generated code
//! will be less performant. You can substitute unit variants with pointers
//! to avoid this situation.
//!
//! ```
//! use enum_ptr::EnumPtr;
//!
//! #[derive(EnumPtr)]
//! #[repr(C, usize)]
//! enum Foo {
//!     A(Box<i64>),
//!     B, // -> B(Option<Box<i64>>)
//! }
//! ```
//!
//! # Limitations
//!
//! Suppose we are deriving from `Foo`, then
//!
//! - `Foo` must be 2 pointers wide.
//!   - If `Foo` is smaller, it is already in the compact representation.
//!   - If `Foo` is larger, this crate cannot compress it into a `usize`.
//! - `Foo` must have a `#[repr(C, usize)]`.
//!   - According to the [RFC] and the [Rust Reference], `#[repr(C, usize)]`
//!     guarantees the memory layout and discriminant values. Thus, we can
//!     safely transmute between two representations.
//! - Each variant of `Foo` must have enough alignment to store the tag.
//! - Each variant of `Foo` must have at most one field.
//!
//! Any violation of these rules will trigger a compilation error except
//! the alignment rule. If not, please file an issue.
//!
//! If some variant has no enough alignment, it will trigger a run-time panic.
//! Otherwise, assertions will be optimized away.
//!
//! [RFC]: https://github.com/rust-lang/rfcs/blob/master/text/2195-really-tagged-unions.md
//! [Rust Reference]: https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-fieldless-enumerations

#![cfg_attr(not(feature = "std"), no_std)]

mod aligned;
mod compact;
mod enum_repr;

pub use aligned::*;
pub use compact::*;
pub use enum_repr::*;

pub use enum_ptr_derive::EnumPtr;
