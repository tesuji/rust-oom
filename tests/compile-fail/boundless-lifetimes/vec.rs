use oom::{NonEmptySlice, NonEmptyVec};

fn main() {
    let vec = NonEmptyVec::<u8>::from_vec(vec![0]);
    let bad: NonEmptySlice<'static, _> = vec.as_nonempty_slice();
    drop(vec);
    let _dangling = bad.first();
}
