# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
# Guiding Principles

* Changelogs are for _humans_, not machines.
* There should be an entry for every single version.
* The same types of changes should be grouped.
* Versions and sections should be linkable.
* The latest version comes first.
* The release date of each version is displayed.
* Mention whether you follow Semantic Versioning.

# Types of changes

* `Added` for new features.
* `Changed` for changes in existing functionality.
* `Deprecated` for soon-to-be removed features.
* `Removed` for now removed features.
* `Fixed` for any bug fixes.
* `Security` in case of vulnerabilities.

# REMEMBER to update the link to the changes between versions.
-->

## [v0.3.0] - 2020-09-02

### Breaking changes

* `NonEmptySlice::from_ref` is not a const fn anymore, due to a refactoring.

### Security

* Fix multiple mutable aliases in `NonEmptyMutSlice::first_mut`. Thyanks @HeroicKatora again.

## [v0.2.0] - 2020-09-02

### Breaking changes

* Remove `NonEmptySlice::fist_mut` (unsound). It is slipped through experimental
  through I intended to remove it.
* Boundless lifetime with `impl<'a, T> for NonEmptyVec`. Thanks @HeroicKatora for reporting
  and helping me adding tests for it.

## [v0.1.0] - 2020-09-02 (yanked)

### Add

* `NonEmptyVec`

### Breaking changes

* `as_mut_ptr` and `as_mut_slice` now require `&mut self`.
* Remove `Clone` impl for `NonEmptyMutSlice`.
  It is kinda unsound for multiple mutable references.

## [v0.0.1] - 2020-09-02 (yanked)

First release

[v0.3.0]: https://github.com/lzutao/rust-oom/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/lzutao/rust-oom/compare/v0.1.0...v0.2.0
[v0.1.0]: https://github.com/lzutao/rust-oom/compare/v0.0.1...v0.1.0
[v0.0.1]: https://github.com/lzutao/rust-oom/releases/tag/v0.0.1
