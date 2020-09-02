use oom::NonEmptyMutSlice;
fn main() {
    let mut foo = String::from("hello");
    let mut s = NonEmptyMutSlice::from_mut(&mut foo);
    let bad: &'static [_] = s.as_mut_slice();
    drop(foo);
    let _dangling = bad.first();
}
