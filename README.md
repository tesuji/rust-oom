# oom - One Or Many slice types

[![Build Status][actions-badge]][actions-url]
[![Documentation](https://docs.rs/oom/badge.svg)](https://docs.rs/oom)
[![Crates.io](https://img.shields.io/crates/v/oom.svg)](https://crates.io/crates/oom)

Took inspirations from [the "Parse, don’t validate" article][pdv].

This package defines two types: `NonEmptySlice` and `NonEmptyMutSlice`.
Note that those types are borrow types (not owned type). So you cannot
use them without borrow contents from array, slice or `Vec`.
Those types don't implement trait `Deref` or `DerefMut`, it is intentional to
avoid confusion when resolving methods. If you want `&[T]`, consider using
`as_slice` or `as_mut_slice`.

Perhaps maybe in the future we will have `NonEmptyVec`, but
there are design questions about semantics when `vec.pop()`.

The size of `NonEmptySlice` and `NonEmptyMutSlice` are the same as `&[T]`.

The differences from `&[T]`:
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
oom = "0.0.1"
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
