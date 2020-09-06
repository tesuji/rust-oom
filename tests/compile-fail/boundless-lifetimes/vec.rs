use oom::{NonEmptySlice, NonEmptyVec};

fn main() {
    let vec = NonEmptyVec::<u8>::from_vec(vec![0]);
    let bad: &'static NonEmptySlice<_> = &vec;
    drop(vec);
    let _dangling = bad.first();
}
