use core::cmp::Ordering;
use core::marker::PhantomData;
use core::mem::size_of;
use core::num::NonZeroUsize;
use core::slice;

/// A non-empty slice type, counterpart of `&[T]`.
pub struct NonEmptySlice<'a, T: Sized> {
    ptr: *const T,
    len: NonZeroUsize,
    lt: PhantomData<&'a T>,
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
            Self {
                ptr: self.ptr,
                len: self.len,
                lt: self.lt,
            }
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

    unsafe impl<'a, T: Send> Send for NonEmptySlice<'a, T> {}
    unsafe impl<'a, T: Sync> Sync for NonEmptySlice<'a, T> {}
};

impl<'a, T: Sized> NonEmptySlice<'a, T> {
    pub(crate) const unsafe fn from_raw_parts(ptr: *const T, len: usize) -> Self {
        Self {
            ptr,
            len: NonZeroUsize::new_unchecked(len),
            lt: PhantomData,
        }
    }

    /// Converts a `&T` into a `NonEmptySlice`.
    pub const fn from_ref(e: &'a T) -> Self {
        unsafe { Self::from_raw_parts(e as *const T, 1) }
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

        let ptr = slice.as_ptr();
        let len = slice.len();
        Some(unsafe { Self::from_raw_parts(ptr, len) })
    }

    /// Returns a raw pointer to the slice's buffer.
    pub fn as_ptr(&self) -> *const T {
        self.ptr
    }

    /// Returns a `&[T]` containing entire `NonEmptySlice`.
    pub fn as_slice(&self) -> &'a [T] {
        unsafe { slice::from_raw_parts(self.ptr, self.len.get()) }
    }

    /// Returns the number of elements in the slice.
    pub const fn len(&self) -> NonZeroUsize {
        self.len
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
    pub fn first(&self) -> &'a T {
        unsafe { &*(self.ptr) }
    }

    /// Returns a mutable pointer to the first element of the slice.
    ///
    /// ```
    /// # use oom::NonEmptySlice;
    /// let arr = &mut [10, 40, 30];
    /// let mut s = NonEmptySlice::from_slice(arr);
    /// *s.first_mut() = 42;
    /// assert_eq!(arr, &[42, 40, 30]);
    /// ```
    pub fn first_mut(&mut self) -> &'a mut T {
        unsafe { &mut *(self.ptr as *mut T) }
    }

    /// Returns the last element of the slice.
    ///
    /// ```
    /// # use oom::NonEmptySlice;
    /// let s = NonEmptySlice::from_slice(&[10, 40, 30]);
    /// assert_eq!(s.last(), &30);
    /// ```
    pub fn last(&self) -> &'a T {
        let last_idx = self.len.get() - 1;
        unsafe { &*(self.ptr.add(last_idx)) }
    }

    /// Returns the first and all the rest of the elements of the slice.
    ///
    /// ```
    /// # use oom::NonEmptySlice;
    /// let s = NonEmptySlice::from_slice(&[10, 40, 30]);
    /// assert_eq!(s.split_first(), (&10, &[40, 30][..]));
    /// ```
    pub fn split_first(&self) -> (&'a T, &'a [T]) {
        let ptr = self.ptr;
        let first = unsafe { &*ptr };
        let rest = unsafe {
            let ptr = ptr.add(1);
            let len = self.len().get() - 1;
            slice::from_raw_parts(ptr, len)
        };
        (first, rest)
    }

    /// Returns the last and all the rest of the elements of the slice.
    ///
    /// ```
    /// # use oom::NonEmptySlice;
    /// let s = NonEmptySlice::from_slice(&[10, 40, 30]);
    /// assert_eq!(s.split_last(), (&30, &[10, 40][..]));
    /// ```
    pub fn split_last(&self) -> (&'a T, &'a [T]) {
        let ptr = self.ptr;
        let len = self.len().get() - 1;
        let rest = unsafe { slice::from_raw_parts(ptr, len) };
        let last = unsafe { &*(ptr.add(len)) };
        (last, rest)
    }
}
