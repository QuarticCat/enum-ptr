# Enum Ptr

[![crates.io](https://img.shields.io/crates/v/enum-ptr)](https://crates.io/crates/enum-ptr)
[![docs.rs](https://img.shields.io/badge/docs.rs-enum--ptr-latest)](https://docs.rs/enum-ptr)

This crate provides a custom derive macro `EnumPtr` to generate bridges between an enum `T` and `Compact<T>` with minimum cost. `Compact<T>` is the compact representation of `T`, and it is only one pointer wide.
In other words, this crate is a library for defining tagged pointers in ergonomic way, even supporting different pointer types (`&`, `Box`, `Arc`, etc) as different `enum` variants.

For example, the following code

```rust
use enum_ptr::EnumPtr;

#[derive(EnumPtr)]
#[repr(C, usize)]
enum Foo<'a> {
    A(&'a i32),
    B(Box<i32>),
}
```

will generate

```rust
impl<'a> From<Foo<'a>> for Compact<Foo<'a>> {
    // ...
}

impl<'a> From<Compact<Foo<'a>>> for Foo<'a> {
    // ...
}
```

Since `&i32` and `Box<i32>` are aligned by 4 bytes, the lowest 2 bits of them are always zeros. `Compact<Foo<'a>>` utilizes these bits to store the tag (discriminant).

## Features

- No need to write unsafe pointer operations
- Supports various types and can be extended
- Supports `no_std`
- Minimum type conversion cost
- Passes `cargo +nightly miri test` with strict provenance enabled.

## Testing

```console
$ cargo test
$ cargo +nightly miri test
```

## Credits

- Thanks to [@oxalica](https://github.com/oxalica) for reviewing this crate and providing a lot of helpful suggestions.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](/LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](/LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.
