//! This crate provides a custom derive macro [`EnumPtr`] to generate bridges
//! between an enum `T` and [`Compact<T>`] with minimum cost.
//!
//! [`Compact<T>`] is the compact representation of `T`, and it is only one
//! pointer wide. This is viable because some types' low bits are always zeros.
//! [`Compact<T>`] utilizes these bits to store the tag (discriminant value).
//!
//! # Examples
//!
//! ```
//! use enum_ptr::{Aligned, Compact, EnumPtr, ShiftUsize, Unit};
//!
//! # #[derive(Debug, PartialEq, Eq, Clone)]
//! #[derive(EnumPtr)]
//! #[repr(C, usize)] // required
//! enum Foo<'a, T: Aligned> {
//!     A(T),             // supports any `T: Aligned`
//!     B(&'a u64),
//!     C(Unit),          // use `Unit` for unit variants
//!     D(ShiftUsize<3>), // you can even use non-pointers
//! #    #[cfg(feature = "alloc")]
//!     E(Box<i64>),
//! }
//!
//! let compact_foo: Compact<_> = Foo::A(&1u64).into();
//! let original_foo: Foo<_> = compact_foo.into();
//! #
//! # let test = |f: Foo<&u64>| assert_eq!(f.clone(), Foo::from(Compact::from(f)));
//! # test(Foo::A(&0));
//! # test(Foo::B(&1));
//! # test(Foo::C(Unit::new()));
//! # test(Foo::D(ShiftUsize::new(2)));
//! # #[cfg(feature = "alloc")]
//! # test(Foo::E(Box::new(3)));
//! ```
//!
//! # Usage
//!
//! This crate provides multiple flavors of APIs.
//!
//! ## Flavor 1: `CompactCopy`
//!
//! If your enum type is [`Copy`] (e.g., consists of only `&T`s), you can
//! convert it to [`CompactCopy`]. Each time you need to use it, just copy
//! and [`extract`](CompactCopy::extract) it. Easy-peasy!
//!
//! Sadly, due to language limitations, we cannot combine [`Compact`] and
//! [`CompactCopy`] into one type.
//!
//! <details>
//! <summary>Click to show examples</summary>
//!
//! ```
//! use enum_ptr::{CompactCopy, EnumPtr};
//!
//! #[derive(EnumPtr, Debug, Clone, Copy, PartialEq, Eq)]
//! #[enum_ptr(copy)] // required
//! #[repr(C, usize)]
//! enum Foo<'a, 'b> {
//!     A(&'a i32),
//!     B(&'b u32),
//! }
//!
//! let foo: CompactCopy<_> = Foo::A(&1).into();
//! assert_eq!(foo.extract(), Foo::A(&1));
//! assert_ne!(foo.extract(), Foo::B(&2));
//! ```
//! </details>
//!
//! ## Flavor 2: `get_ref` & `get_mut`
//!
//! If your enum type is not [`Copy`], and you happens to only have references
//! to the compact value, you can use [`get_ref`] and [`get_mut`] to get
//! references to **the object that it points to**.
//!
//! For example, if you hold a compact `Box<T>`, you can use these APIs to
//! access `&T` and `&mut T`. Since there's no `Box<T>` in the memory (but only
//! its compact form), we cannot create `&Box<T>` and `&mut Box<T>`. The
//! target types are specified by [`FieldDeref`] and [`FieldDerefMut`].
//!
//! <details>
//! <summary>Click to show examples</summary>
//!
//! ```
//! # #[cfg(feature = "alloc")] {
//! use enum_ptr::{get_mut, get_ref, Compact, EnumPtr};
//!
//! #[derive(EnumPtr)]
//! #[repr(C, usize)]
//! enum Foo {
//!     A(Box<i32>),
//!     B(Box<u32>),
//! }
//!
//! let mut foo: Compact<_> = Foo::A(Box::new(1)).into();
//! assert_eq!(get_ref!(foo, Foo::A), Some(&1));
//! assert_eq!(get_mut!(foo, Foo::A), Some(&mut 1));
//! # }
//! ```
//! </details>
//!
//! ## Flavor 3: `borrow` & `borrow_mut`
//!
//! [`get_ref`] and [`get_mut`] can be troublesome if you want to deal with
//! multiple variants at together. In that case, you can use
//! [`borrow`](Compact::borrow) and [`borrow_mut`](Compact::borrow_mut). They
//! will return derived reference types that you can `match`.
//!
//! Check the documentation of [`EnumPtr`] for more details.
//!
//! <details>
//! <summary>Click to show examples</summary>
//!
//! ```
//! # #[cfg(feature = "alloc")] {
//! use enum_ptr::{Compact, EnumPtr};
//!
//! #[derive(EnumPtr, Debug)]
//! #[enum_ptr(borrow, borrow_mut)] // required
//! #[repr(C, usize)]
//! enum Foo {
//!     A(Box<i32>),
//!     B(Option<Box<u32>>),
//! }
//!
//! // enum FooRef<'enum_ptr> {
//! //     A(&'enum_ptr i32),
//! //     B(Option<&'enum_ptr u32>),
//! // }
//!
//! // enum FooRefMut<'enum_ptr> {
//! //     A(&'enum_ptr mut i32),
//! //     B(Option<&'enum_ptr mut u32>),
//! // }
//!
//! let mut foo: Compact<_> = Foo::A(Box::new(1)).into();
//! match foo.borrow() {
//!     FooRef::A(inner) => assert_eq!(inner, &1),
//!     _ => unreachable!(),
//! }
//! match foo.borrow_mut() {
//!     FooRefMut::A(inner) => assert_eq!(inner, &mut 1),
//!     _ => unreachable!(),
//! }
//! # }
//! ```
//! </details>
//!
//! ## Flavor 4: `map_ref` & `map_mut` *(legacy)*
//!
//! [`map_ref`](Compact::map_ref) and [`map_mut`](Compact::map_mut) will create
//! temporary objects that drop as soon as your closure ends. They are
//! important for internal implementations, but less useful for lib users.
//!
//! <details>
//! <summary>Click to show examples</summary>
//!
//! ```
//! # #[cfg(feature = "alloc")] {
//! use enum_ptr::{Compact, EnumPtr};
//!
//! #[derive(EnumPtr, Debug, PartialEq, Eq)]
//! #[repr(C, usize)]
//! enum Foo {
//!     A(Box<i32>),
//!     B(Box<u32>),
//! }
//!
//! let mut foo: Compact<_> = Foo::A(Box::new(1)).into();
//!
//! let result = foo.map_ref(|f| match f {
//!     Foo::A(r) => **r,
//!     _ => unreachable!(),
//! });
//! assert_eq!(result, 1);
//!
//! unsafe {
//!     foo.map_mut(|f| match f {
//!         Foo::A(r) => **r = 2,
//!         _ => unreachable!(),
//!     });
//! }
//! assert_eq!(foo.extract(), Foo::A(Box::new(2)));
//! # }
//! ```
//! </details>
//!
//! ## Extension
//!
//! All important traits are public. You can implement them for your own types.
//!
//! - To make your types available in [`EnumPtr`], implement [`Aligned`].
//! - To make your types available in [`get_ref`] / [`get_mut`] and
//!   `#[enum_ptr(borrow)]` / `#[enum_ptr(borrow_mut)]`, implement
//!   [`FieldDeref`] / [`FieldDerefMut`].
//! - Unsatisfied with derived reference types? Implement [`CompactBorrow`] /
//!   [`CompactBorrowMut`] by hand.
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
//! - `alloc` *(default)* --- `Box`, `Rc` and `Arc` support

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
///         name = "FooRef",     // default: ident + "Ref"
///         derive(Clone, Copy), // default: none
///     ),
///     borrow_mut( // derives a reference type and `impl CompactBorrowMut`
///         name = "FooRefMut",  // default: ident + "RefMut"
///         derive(Debug),       // default: none
///     ),
/// )]
/// #[repr(C, usize)]
/// enum Foo {
///     // `borrow` / `borrow_mut` requires all unskipped fields
///     // to implement `FieldDeref` / `FieldDerefMut`
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
