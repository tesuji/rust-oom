use oom::{NonEmptyMutSlice, NonEmptyVec};
use std::path::Path;

#[test]
fn test_muts() {
    let config_dirs = &mut [
        Path::new("/home/user/.config/nvim"),
        Path::new("/etc/nvim"),
        Path::new("/usr/share/nvim"),
    ];
    let mut s = NonEmptyMutSlice::from_slice_checked(config_dirs).unwrap();

    assert_eq!(s.len().get(), 3);
    assert_eq!(s.is_empty(), false);

    let first = Path::new("/home/user/.config/neovim");
    let last = Path::new("/home/user/.config/vim");

    {
        *(s.first_mut()) = first;
        assert_eq!(*s.first(), first);
    }

    {
        *(s.last_mut()) = last;
        assert_eq!(*s.last(), last);
    }

    let arr = &mut [0, 1, 2];
    let mut s = NonEmptyMutSlice::from_slice_checked(arr).unwrap();

    {
        let (first, rest) = s.split_first_mut();
        *first = 42;
        rest[0] = 2;
        rest[1] = 3;
        assert_eq!(s.as_slice(), &[42, 2, 3][..]);
    }

    {
        let (last, rest) = s.split_last_mut();
        *last = 0;
        rest[0] = 42;
        rest[1] = 42;
        assert_eq!(s.as_slice(), &[42, 42, 0][..]);
    }

    let v = vec![1, 2, 3];
    let v = NonEmptyVec::from_vec_checked(v).unwrap();
    assert_eq!(v.as_slice(), &[1, 2, 3]);
    drop(v);
    drop(v);

    let v = Vec::<u32>::with_capacity(42);
    match NonEmptyVec::from_vec_checked(v) {
        Ok(_) => panic!("slice is empty"),
        Err(v) => assert!(v.is_empty()),
    }
}
