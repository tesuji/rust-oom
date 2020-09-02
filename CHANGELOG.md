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
 -->

## [0.1.0] - 2020-09-02
### Add

* `NonEmptyVec`

### Breaking changes

* `as_mut_ptr` and `as_mut_slice` now require `&mut self`.

## [0.0.1] - 2020-09-02

First release

[0.1.0]: https://github.com/lzutao/rust-oom/compare/v0.0.1...v0.1.0
[0.0.1]: https://github.com/lzutao/rust-oom/releases/tag/v0.0.1
