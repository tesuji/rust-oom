use core::cmp::Ordering;
use core::hint::unreachable_unchecked;
use core::mem::size_of;
use core::num::NonZeroUsize;
use core::slice;
#[cfg(feature = "std")]
use std::fmt;

// FIXME: Use unsized `[T]` as inner type (see loaf crate)
// and return `&NonEmptySlice` or `&mut NonEmptySlice`.
// That would simplify alot of code. But it requires
// unstable ZST casts and nightly in const contexts.
/// A non-empty slice type, counterpart of `&[T]`.
pub struct NonEmptySlice<'a, T: Sized> {
    pub(crate) inner: &'a [T],
}

const _SIZE: () = {
    const FOO: [(); 1] = [()];
    const SIZE: usize = size_of::<NonEmptySlice<'_, &str>>();
    #[cfg(target_pointer_width = "64")]
    let idx = !(SIZE == 16) as usize;
    #[cfg(target_pointer_width = "32")]
    let idx = !(SIZE == 8) as usize;
    FOO[idx]
};

const _BUILTIN_TRAITS: () = {
    impl<'a, T: Clone> Clone for NonEmptySlice<'a, T> {
        fn clone(&self) -> Self {
            Self { inner: self.inner }
        }
    }

    impl<'a, T: Copy> Copy for NonEmptySlice<'a, T> {}

    impl<'a, T: Eq> Eq for NonEmptySlice<'a, T> {}

    impl<'a, T: PartialEq> PartialEq for NonEmptySlice<'a, T> {
        fn eq(&self, other: &Self) -> bool {
            self.as_slice().eq(other.as_slice())
        }
    }

    impl<'a, T: Ord> Ord for NonEmptySlice<'a, T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.as_slice().cmp(other.as_slice())
        }
    }

    impl<'a, T: PartialOrd> PartialOrd for NonEmptySlice<'a, T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.as_slice().partial_cmp(other.as_slice())
        }
    }

    impl<'a, T> AsRef<[T]> for NonEmptySlice<'a, T> {
        fn as_ref(&self) -> &[T] {
            self.as_slice()
        }
    }

    #[cfg(feature = "std")]
    impl<'a, T: fmt::Debug> fmt::Debug for NonEmptySlice<'a, T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
            fmt::Debug::fmt(self.as_slice(), f)
        }
    }
};

impl<'a, T: Sized> NonEmptySlice<'a, T> {
    /// Converts a `&T` into a `NonEmptySlice`.
    pub fn from_ref(e: &'a T) -> Self {
        Self {
            inner: slice::from_ref(e),
        }
    }

    /// Converts a `&[T]` into a `NonEmptySlice`.
    ///
    /// # Panics
    ///
    /// This function will panic if the passed slice is empty.
    pub fn from_slice(slice: &'a [T]) -> Self {
        Self::from_slice_checked(slice).expect("slice shouldn't be empty")
    }

    /// Converts a `&[T]` into a `NonEmptySlice`.
    /// Returns `None` if the passed slice is empty.
    pub const fn from_slice_checked(slice: &'a [T]) -> Option<Self> {
        if slice.is_empty() {
            return None;
        }

        Some(Self { inner: slice })
    }

    /// Returns a raw pointer to the slice's buffer.
    pub fn as_ptr(&self) -> *const T {
        self.inner.as_ptr()
    }

    /// Returns a `&[T]` containing entire `NonEmptySlice`.
    pub fn as_slice(&self) -> &[T] {
        self.inner
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
        match self.inner {
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
        match self.inner {
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
        match self.inner {
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
        match self.inner {
            [rest @ .., last] => (last, rest),
            [] => unsafe { unreachable_unchecked() },
        }
    }
}
