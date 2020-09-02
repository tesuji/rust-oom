fn main() {
    let mut t = 0;
    let mut slice = oom::NonEmptyMutSlice::<u8>::from_mut(&mut t);
    let first = slice.first_mut();
    let second = slice.first_mut();
    assert_eq!(first as *mut _, second as *mut _);
    *first = 0;
    *second = 1;
    assert_eq!(*first, 1, "UB, written here through second.");
}
