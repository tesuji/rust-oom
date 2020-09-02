/*!
Took inspirations from [the "Parse, don’t validate" article][1].

[1]: https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/

More introduction is in the README.
*/

#![no_std]
#![doc(html_root_url = "https://docs.rs/oom/0.1.0")]
#![warn(rust_2018_idioms)]

#[cfg(feature = "slice")]
mod slice;
#[cfg(feature = "vec")]
mod vec;

#[cfg(feature = "slice")]
pub use slice::share::NonEmptySlice;
#[cfg(feature = "slice")]
pub use slice::unique::NonEmptyMutSlice;

#[cfg(feature = "vec")]
pub use vec::NonEmptyVec;

#[cfg(doctest)]
mod doctests;
