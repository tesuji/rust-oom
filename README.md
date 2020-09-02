# oom - One Or Many slice types

[![Build Status][actions-badge]][actions-url]
[![Documentation](https://docs.rs/oom/badge.svg)](https://docs.rs/oom)
[![Crates.io](https://img.shields.io/crates/v/oom.svg)](https://crates.io/crates/oom)

Took inspirations from [the "Parse, don’t validate" article][pdv].

This package defines three types: `NonEmptySlice`, `NonEmptyMutSlice`
and `NonEmptyVec`. Those types don't implement `Deref` or `DerefMut` trait,
it is intentional to avoid confusion when resolving methods.
If you want `&[T]`, consider using `as_slice` or `as_mut_slice` methods.

`NonEmptySlice` and `NonEmptyMutSlice`:

* are borrow types (not owned type).
* are counterparts of `&[T]` and `&mut [T]`.
* have same size and similar niche as `&[T]`
* cannot be used without borrow contents from array, slice or `Vec`.

`NonEmptyVec`:

* is an owned types, counterparts of `Vec<T>`.
* doesn't have `push` and `pop` methods, because those are fallible operations.
  I had to deal with unsafe codes that I am not confident if I want to implement them.

The differences from `&[T]` and `Vec<T>`:
* `.len()` returns std's `NonZeroUsize`.
* `.is_empty()` is always false.
* These methods don't return `None`:
  - `first`
  - `first_mut`
  - `last`
  - `last_mut`
  - `split_first`
  - `split_last`
  - `split_first_mut`
  - `split_last_mut`

## Notable features

* `#![no_std]`
* no external dependencies
* no macros
* instant build time.

## Supported Rust versions

Latest stable. I use the new conditional flows in `const fn` available only in Rust v1.46.0.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
oom = "0.3.0"
```

Or assuming you installed [`cargo-edit`][edit], use:

```
cargo add oom
```

## Thanks for inspirations from

* https://github.com/cloudhead/nonempty (MIT license)
* https://github.com/yihuang/non-empty-vec (MIT license)

## License

All the code in this repository is released under the MIT License,
for more information read the COPYRIGHT file.

<!-- Reference links -->
[actions-badge]: https://github.com/lzutao/rust-oom/workflows/Rust/badge.svg?branch=master
[actions-url]: https://github.com/lzutao/rust-oom/actions
[edit]: https://github.com/killercup/cargo-edit
[pdv]: https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/
