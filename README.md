# Enum Ptr (WIP)

This crate provides a custom derive macro `EnumPtr` to automatically generate bridges between `T` and `Compact<T>` with minimum cost. For example, the following code

```rust
use enum_ptr::EnumPtr;

#[derive(EnumPtr)]
#[repr(C, usize)]
enum Foo<'a> {
    A(&'a i32),
    B(Option<Box<i32>>),
}
```

will generate

```rust
impl<'a> From<Foo<'a>> for enum_ptr::Compact<Foo<'a>> {
    // ...
}

impl<'a> From<enum_ptr::Compact<Foo<'a>>> for Foo<'a> {
    // ...
}
```

where `Compact<Foo<'a>>` is the compact representation of `Foo<'a>`. It is only one pointer wide.

Since `&i32` and `Box<i32>` are aligned by 4 bytes, the lowest 2 bits of them are always zeros. `Compact<Foo<'a>>` utilizes these bits to store the tag (discriminant value).

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

See [docs.rs](https://docs.rs/enum-ptr/latest/enum-ptr/)

## Credits

- Thanks to [@oxalica](https://github.com/oxalica) for reviewing this crate and providing a lot of helpful suggestions.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](/LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](/LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.
