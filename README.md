# Enum Ptr

This crate provides a custom derive macro `EnumPtr` to automatically generate compact representations of `enum`s of pointers and conversions between them with minimum cost. For example, the following code

```rust
use enum_ptr::EnumPtr;

#[derive(EnumPtr)]
#[repr(C, usize)]
enum Foo<'a> {
    A(&'a i32),
    B(Option<Box<i32>>),
    C,
}
```

will generate

```rust
struct CompactFoo<'a> {
    data: enum_ptr::Private<usize>,
    phantom: PhantomData<Foo<'a>>,
}

impl<'a> From<CompactFoo<'a>> for Foo<'a> {
    // ...
}

impl<'a> From<Foo<'a>> for CompactFoo<'a> {
    // ...
}
```

It utilizes the fact that if `T` is aligned by 2^N bytes, then the lowest N bits of pointers to `T` must be zeros. We can use these bits to store information such as tags (discriminant values).

## Usage

### Dependencies

```toml
[dependencies]
enum-ptr = "*"
```

This crate also supports `no_std`.

```toml
[dependencies]
enum-ptr = { version = "*", default-features = false }
```

### Code

Define a `enum` with different pointers or units and add `#[derive(EnumPtr)]` and `#[repr(C, usize)]` on top of it.

```rust
use enum_ptr::EnumPtr;

#[derive(EnumPtr)]
#[repr(C, usize)]
enum Foo<'a, 'b, T> {
    A(&'a i64),
    B { ptr: &'b mut T },
    C(Option<Box<i64>>),
    D(),
    E {},
    F,
}
```

Then you can convert between the original `enum` and the compact representation by

```rust
let foo = /* ... */;
let compact_foo = CompactFoo::from(foo);
let original_foo = Foo::from(compact_foo);
```

The `enum` can have generic parameters. Its variants can be named (`X { ... }`), unnamed (`X(...)`), or units (`X`). Each variant can have at most one field.

As long as a type implements the `Compactable` trait, it can be used as the field type. You can also implement this trait for your own pointer types.

## Limitations

Suppose we are deriving from `Foo`, then

- `Foo` must be 2 pointers wide.
  - If `Foo` is smaller, it is already in the compact representation.
  - If `Foo` is larger, this crate cannot compress it into a `usize`.
- `Foo` must have a `#[repr(C, usize)]`.
  - According to the [RFC](https://github.com/rust-lang/rfcs/blob/master/text/2195-really-tagged-unions.md) and the [Rust Reference](https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-fieldless-enumerations), `#[repr(C, usize)]` guarantees the memory layout and discriminant values. Thus, we can safely transmute between two representations.
- Each variant of `Foo` must have enough alignment to store the tag.
- Each variant of `Foo` must have at most one field (multiple fields are not supported for now).

Any violation of these rules will trigger a compilation error except alignment checks. Otherwise, please file an issue.

If some variant has no enough alignment, it will trigger a run-time panic. Or assertions will be optimized away. There is no extra run-time cost.

## TODO

- Rename option
- Derive option

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](/LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](/LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.
