use core::cmp::Ordering;
use core::hint::unreachable_unchecked;
use core::mem::size_of;
use core::num::NonZeroUsize;
use core::slice;

// FIXME: Use unsized `[T]` as inner type (see loaf crate)
// and return `&NonEmptySlice` or `&mut NonEmptySlice`.
// That would simplify alot of code. But it requires
// unstable ZST casts and nightly in const contexts.
/// A non-empty slice type, counterpart of `[T]`.
#[repr(C)]
pub struct NonEmptySlice<T: Sized> {
    pub(crate) inner: [T],
}

const _SIZE: () = {
    const FOO: [(); 1] = [()];
    const SIZE: usize = size_of::<&NonEmptySlice<&str>>();
    #[cfg(target_pointer_width = "64")]
    let idx = !(SIZE == 16) as usize;
    #[cfg(target_pointer_width = "32")]
    let idx = !(SIZE == 8) as usize;
    FOO[idx]
};

const _BUILTIN_TRAITS: () = {
    impl<T: Eq> Eq for NonEmptySlice<T> {}

    impl<T: PartialEq> PartialEq for NonEmptySlice<T> {
        fn eq(&self, other: &Self) -> bool {
            self.as_slice().eq(other.as_slice())
        }
    }

    impl<T: Ord> Ord for NonEmptySlice<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.as_slice().cmp(other.as_slice())
        }
    }

    impl<T: PartialOrd> PartialOrd for NonEmptySlice<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.as_slice().partial_cmp(other.as_slice())
        }
    }

    impl<T> AsRef<[T]> for NonEmptySlice<T> {
        fn as_ref(&self) -> &[T] {
            self.as_slice()
        }
    }
};

impl<T: Sized> NonEmptySlice<T> {
    /// Converts a `&T` into a `NonEmptySlice`.
    pub fn from_ref(e: &T) -> &Self {
        unsafe { &*(slice::from_ref(e) as *const [T] as *const Self) }
    }

    /// Converts a `&T` into a `NonEmptySlice`.
    pub fn from_mut(e: &mut T) -> &mut Self {
        unsafe { &mut *(slice::from_mut(e) as *mut [T] as *mut Self) }
    }

    /// Converts a `&[T]` into a `NonEmptySlice`.
    ///
    /// # Panics
    ///
    /// This function will panic if the passed slice is empty.
    pub fn from_slice(slice: &[T]) -> &Self {
        Self::from_slice_checked(slice).expect("slice shouldn't be empty")
    }

    /// Converts a `&mut [T]` into a `NonEmptySlice`.
    ///
    /// # Panics
    ///
    /// This function will panic if the passed slice is empty.
    pub fn from_mut_slice(slice: &mut [T]) -> &mut Self {
        Self::from_mut_slice_checked(slice).expect("slice shouldn't be empty")
    }

    /// Converts a `&[T]` into a `NonEmptySlice`.
    /// Returns `None` if the passed slice is empty.
    pub fn from_slice_checked(slice: &[T]) -> Option<&Self> {
        if slice.is_empty() {
            return None;
        }
        Some(unsafe { &*(slice as *const [T] as *const Self) })
    }

    /// Converts a `&mut [T]` into a `NonEmptySlice`.
    /// Returns `None` if the passed slice is empty.
    pub fn from_mut_slice_checked(slice: &mut [T]) -> Option<&mut Self> {
        if slice.is_empty() {
            return None;
        }
        Some(unsafe { &mut *(slice as *mut [T] as *mut Self) })
    }

    /// Returns a raw pointer to the slice's buffer.
    pub fn as_ptr(&self) -> *const T {
        self.inner.as_ptr()
    }

    /// Returns a mutable raw pointer to the slice's buffer.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.inner.as_mut_ptr()
    }

    /// Returns a `&[T]` containing entire `NonEmptySlice`.
    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }

    /// Returns a `&mut [T]` containing entire `NonEmptySlice`.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.inner
    }

    /// Returns the number of elements in the slice.
    pub const fn len(&self) -> NonZeroUsize {
        unsafe { NonZeroUsize::new_unchecked(self.inner.len()) }
    }

    /// Always returns `false` because the slice is non-empty.
    pub const fn is_empty(&self) -> bool {
        false
    }

    /// Returns the first element of the slice.
    ///
    /// ```
    /// # use oom::NonEmptySlice;
    /// let s = NonEmptySlice::from_slice(&[10, 40, 30]);
    /// assert_eq!(s.first(), &10);
    /// ```
    pub fn first(&self) -> &T {
        match &self.inner[..] {
            [first, ..] => first,
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns a mutable pointer to the first element of the slice.
    ///
    /// ```
    /// # use oom::NonEmptySlice;
    /// let arr = &mut [10, 40, 30];
    /// let s = NonEmptySlice::from_mut_slice(arr);
    /// *s.first_mut() = 42;
    /// assert_eq!(arr, &[42, 40, 30]);
    /// ```
    pub fn first_mut(&mut self) -> &mut T {
        match &mut self.inner[..] {
            [first, ..] => first,
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the last element of the slice.
    ///
    /// ```
    /// # use oom::NonEmptySlice;
    /// let s = NonEmptySlice::from_slice(&[10, 40, 30]);
    /// assert_eq!(s.last(), &30);
    /// ```
    pub fn last(&self) -> &T {
        match &self.inner[..] {
            [.., last] => last,
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the last element of the slice.
    ///
    /// ```
    /// # use oom::NonEmptySlice;
    /// let arr = &mut [10, 40, 30];
    /// let s = NonEmptySlice::from_mut_slice(arr);
    /// *s.last_mut() = 42;
    /// assert_eq!(arr, &[10, 40, 42]);
    /// ```
    pub fn last_mut(&mut self) -> &mut T {
        match &mut self.inner[..] {
            [.., last] => last,
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the first and all the rest of the elements of the slice.
    ///
    /// ```
    /// # use oom::NonEmptySlice;
    /// let s = NonEmptySlice::from_slice(&[10, 40, 30]);
    /// assert_eq!(s.split_first(), (&10, &[40, 30][..]));
    /// ```
    pub fn split_first(&self) -> (&T, &[T]) {
        match &self.inner[..] {
            [first, rest @ ..] => (first, rest),
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the first and all the rest of the elements of the slice.
    ///
    /// ```
    /// # use oom::NonEmptySlice;
    /// let arr = &mut [0, 1, 2];
    /// let s = NonEmptySlice::from_mut_slice(arr);
    /// let (first, rest) = s.split_first_mut();
    /// *first = 3;
    /// rest[0] = 4;
    /// rest[1] = 5;
    /// assert_eq!(arr, &[3, 4, 5]);
    /// ```
    pub fn split_first_mut(&mut self) -> (&mut T, &mut [T]) {
        match &mut self.inner[..] {
            [first, rest @ ..] => (first, rest),
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the last and all the rest of the elements of the slice.
    ///
    /// ```
    /// # use oom::NonEmptySlice;
    /// let s = NonEmptySlice::from_slice(&[10, 40, 30]);
    /// assert_eq!(s.split_last(), (&30, &[10, 40][..]));
    /// ```
    pub fn split_last(&self) -> (&T, &[T]) {
        match &self.inner[..] {
            [rest @ .., last] => (last, rest),
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the last and all the rest of the elements of the slice.
    ///
    /// ```
    /// # use oom::NonEmptySlice;
    /// let arr = &mut [0, 1, 2];
    /// let s = NonEmptySlice::from_mut_slice(arr);
    /// let (last, rest) = s.split_last_mut();
    /// *last = 3;
    /// rest[0] = 4;
    /// rest[1] = 5;
    /// assert_eq!(arr, &[4, 5, 3]);
    /// ```
    pub fn split_last_mut(&mut self) -> (&mut T, &mut [T]) {
        match &mut self.inner[..] {
            [rest @ .., last] => (last, rest),
            [] => unsafe { unreachable_unchecked() },
        }
    }
}
