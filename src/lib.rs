/*!
Took inspirations from [the "Parse, don’t validate" article][1].

[1]: https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/

More introduction is in the README.
*/

#![no_std]
#![doc(html_root_url = "https://docs.rs/oom/0.0.1")]
#![warn(rust_2018_idioms)]

mod slice;

pub use slice::share::NonEmptySlice;
pub use slice::unique::NonEmptyMutSlice;
