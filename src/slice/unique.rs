use core::cmp::Ordering;
use core::hint::unreachable_unchecked;
use core::mem::size_of;
use core::num::NonZeroUsize;
use core::slice;
#[cfg(feature = "std")]
use std::fmt;

/// A non-empty mutable slice type, counterpart of `&mut [T]`.
pub struct NonEmptyMutSlice<'a, T: Sized> {
    pub(crate) inner: &'a mut [T],
}

const _SIZE: () = {
    const FOO: [(); 1] = [()];
    const SIZE: usize = size_of::<NonEmptyMutSlice<'_, &str>>();
    #[cfg(target_pointer_width = "64")]
    let idx = !(SIZE == 16) as usize;
    #[cfg(target_pointer_width = "32")]
    let idx = !(SIZE == 8) as usize;
    FOO[idx]
};

const _BUILTIN_TRAITS: () = {
    impl<'a, T: Eq> Eq for NonEmptyMutSlice<'a, T> {}

    impl<'a, T: PartialEq> PartialEq for NonEmptyMutSlice<'a, T> {
        fn eq(&self, other: &Self) -> bool {
            self.as_slice().eq(other.as_slice())
        }
    }

    impl<'a, T: Ord> Ord for NonEmptyMutSlice<'a, T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.as_slice().cmp(other.as_slice())
        }
    }

    impl<'a, T: PartialOrd> PartialOrd for NonEmptyMutSlice<'a, T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.as_slice().partial_cmp(other.as_slice())
        }
    }

    impl<'a, T> AsRef<[T]> for NonEmptyMutSlice<'a, T> {
        fn as_ref(&self) -> &[T] {
            self.as_slice()
        }
    }

    #[cfg(feature = "std")]
    impl<'a, T: fmt::Debug> fmt::Debug for NonEmptyMutSlice<'a, T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
            fmt::Debug::fmt(self.as_slice(), f)
        }
    }
};

impl<'a, T: Sized> NonEmptyMutSlice<'a, T> {
    /// Converts a `&T` into a `NonEmptyMutSlice`.
    pub fn from_mut(e: &'a mut T) -> Self {
        Self {
            inner: slice::from_mut(e),
        }
    }

    /// Converts a `&[T]` into a `NonEmptyMutSlice`.
    ///
    /// # Panics
    ///
    /// This function will panic if the passed slice is empty.
    pub fn from_slice(slice: &'a mut [T]) -> Self {
        Self::from_slice_checked(slice).expect("slice shouldn't be empty")
    }

    /// Converts a `&[T]` into a `NonEmptyMutSlice`.
    /// Returns `None` if the passed slice is empty.
    pub fn from_slice_checked(slice: &'a mut [T]) -> Option<Self> {
        if slice.is_empty() {
            return None;
        }

        Some(Self { inner: slice })
    }

    /// Returns a raw pointer to the slice's buffer.
    pub fn as_ptr(&self) -> *const T {
        self.inner.as_ptr()
    }

    /// Returns an unsafe mutable pointer to the slice's buffer.
    ///
    /// The caller must ensure that the slice outlives the pointer
    /// this function returns, or else it will end up pointing to garbage.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.inner.as_mut_ptr()
    }

    /// Returns a `&[T]` containing entire `NonEmptyMutSlice`.
    pub fn as_slice(&self) -> &[T] {
        self.inner
    }

    /// Returns a mutable slice from this type.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.inner
    }

    /// Returns the number of elements in the slice.
    pub fn len(&self) -> NonZeroUsize {
        unsafe { NonZeroUsize::new_unchecked(self.inner.len()) }
    }

    /// Always returns `false` because the slice is non-empty.
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Returns the first element of the slice.
    ///
    /// ```
    /// # use oom::NonEmptyMutSlice;
    /// let arr = &mut [10, 40, 30];
    /// let s = NonEmptyMutSlice::from_slice(arr);
    /// assert_eq!(s.first(), &10);
    /// ```
    pub fn first(&self) -> &T {
        match self.inner {
            [ref first, ..] => first,
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns a mutable pointer to the first element of the slice.
    ///
    /// ```
    /// # use oom::NonEmptyMutSlice;
    /// let arr = &mut [10, 40, 30];
    /// let mut s = NonEmptyMutSlice::from_slice(arr);
    /// *s.first_mut() = 42;
    /// assert_eq!(arr, &[42, 40, 30]);
    /// ```
    pub fn first_mut(&mut self) -> &mut T {
        match self.inner {
            [first, ..] => first,
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the last element of the slice.
    ///
    /// ```
    /// # use oom::NonEmptyMutSlice;
    /// let arr = &mut [10, 40, 30];
    /// let s = NonEmptyMutSlice::from_slice(arr);
    /// assert_eq!(s.last(), &30);
    /// ```
    pub fn last(&self) -> &T {
        match self.inner {
            [.., ref last] => last,
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the last element of the slice.
    ///
    /// ```
    /// # use oom::NonEmptyMutSlice;
    /// let arr = &mut [10, 40, 30];
    /// let mut s = NonEmptyMutSlice::from_slice(arr);
    /// *s.last_mut() = 42;
    /// assert_eq!(arr, &[10, 40, 42]);
    /// ```
    pub fn last_mut(&mut self) -> &mut T {
        match self.inner {
            [.., last] => last,
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the first and all the rest of the elements of the slice.
    ///
    /// ```
    /// # use oom::NonEmptyMutSlice;
    /// let arr = &mut [10, 40, 30];
    /// let s = NonEmptyMutSlice::from_slice(arr);
    /// assert_eq!(s.split_first(), (&10, &[40, 30][..]));
    /// ```
    pub fn split_first(&self) -> (&T, &[T]) {
        match self.inner {
            [ref first, ref rest @ ..] => (first, rest),
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the first and all the rest of the elements of the slice.
    ///
    /// ```
    /// # use oom::NonEmptyMutSlice;
    /// let arr = &mut [0, 1, 2];
    /// let mut s = NonEmptyMutSlice::from_slice(arr);
    /// let (first, rest) = s.split_first_mut();
    /// *first = 3;
    /// rest[0] = 4;
    /// rest[1] = 5;
    /// assert_eq!(arr, &[3, 4, 5]);
    /// ```
    pub fn split_first_mut(&mut self) -> (&mut T, &mut [T]) {
        match self.inner {
            [first, rest @ ..] => (first, rest),
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the last and all the rest of the elements of the slice.
    ///
    /// ```
    /// # use oom::NonEmptyMutSlice;
    /// let arr = &mut [10, 40, 30];
    /// let s = NonEmptyMutSlice::from_slice(arr);
    /// assert_eq!(s.split_last(), (&30, &[10, 40][..]));
    /// ```
    pub fn split_last(&self) -> (&T, &[T]) {
        match self.inner {
            [ref rest @ .., ref last] => (last, rest),
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the last and all the rest of the elements of the slice.
    ///
    /// ```
    /// # use oom::NonEmptyMutSlice;
    /// let arr = &mut [0, 1, 2];
    /// let mut s = NonEmptyMutSlice::from_slice(arr);
    /// let (last, rest) = s.split_last_mut();
    /// *last = 3;
    /// rest[0] = 4;
    /// rest[1] = 5;
    /// assert_eq!(arr, &[4, 5, 3]);
    /// ```
    pub fn split_last_mut(&mut self) -> (&mut T, &mut [T]) {
        match self.inner {
            [rest @ .., last] => (last, rest),
            [] => unsafe { unreachable_unchecked() },
        }
    }
}
