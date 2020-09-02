/// Boundless lifetime tests.
///
/// ```compile_fail
/// # use oom::{NonEmptyVec, NonEmptySlice};
/// let vec = NonEmptyVec::<u8>::from_vec(vec![0]);
/// let bad: NonEmptySlice<'static, _> = vec.as_nonempty_slice();
/// drop(vec);
/// let dangling = bad.first();
/// ```
extern "C" {}
