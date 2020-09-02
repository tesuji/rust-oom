use core::cmp::Ordering;
use core::marker::PhantomData;
use core::mem::size_of;
use core::num::NonZeroUsize;
use core::slice;

/// An non-empty mutable slice type, counterpart of `&mut [T]`.
pub struct NonEmptyMutSlice<'a, T: Sized> {
    ptr: *mut T,
    len: NonZeroUsize,
    lt: PhantomData<&'a mut T>,
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
    impl<'a, T: Clone> Clone for NonEmptyMutSlice<'a, T> {
        fn clone(&self) -> Self {
            Self {
                ptr: self.ptr,
                len: self.len,
                lt: self.lt,
            }
        }
    }

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
};

impl<'a, T: Sized> NonEmptyMutSlice<'a, T> {
    pub(crate) unsafe fn from_raw_parts_mut(ptr: *mut T, len: usize) -> Self {
        Self {
            ptr,
            len: NonZeroUsize::new_unchecked(len),
            lt: PhantomData,
        }
    }

    /// Converts a `&T` into a `NonEmptyMutSlice`.
    pub fn from_mut(e: &'a mut T) -> Self {
        unsafe { Self::from_raw_parts_mut(e as *mut T, 1) }
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

        let ptr = slice.as_mut_ptr();
        let len = unsafe { NonZeroUsize::new_unchecked(slice.len()) };
        Some(Self {
            ptr,
            len,
            lt: PhantomData,
        })
    }

    /// Returns a raw pointer to the slice's buffer.
    pub fn as_ptr(&self) -> *const T {
        self.ptr
    }

    /// Returns an unsafe mutable pointer to the slice's buffer.
    ///
    /// The caller must ensure that the slice outlives the pointer
    /// this function returns, or else it will end up pointing to garbage.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr
    }

    /// Returns a `&[T]` containing entire `NonEmptyMutSlice`.
    pub fn as_slice(&self) -> &'a [T] {
        unsafe { slice::from_raw_parts(self.ptr as *const T, self.len.get()) }
    }

    /// Returns a mutable slice from this type.
    pub fn as_mut_slice(&mut self) -> &'a mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.len.get()) }
    }

    /// Returns the number of elements in the slice.
    pub fn len(&self) -> NonZeroUsize {
        self.len
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
    pub fn first(&self) -> &'a T {
        unsafe { &*(self.ptr) }
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
    pub fn first_mut(&mut self) -> &'a mut T {
        unsafe { &mut *(self.ptr) }
    }

    /// Returns the last element of the slice.
    ///
    /// ```
    /// # use oom::NonEmptyMutSlice;
    /// let arr = &mut [10, 40, 30];
    /// let s = NonEmptyMutSlice::from_slice(arr);
    /// assert_eq!(s.last(), &30);
    /// ```
    pub fn last(&self) -> &'a T {
        let last_idx = self.len.get() - 1;
        unsafe { &*(self.ptr.add(last_idx)) }
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
    pub fn last_mut(&mut self) -> &'a mut T {
        let last_idx = self.len.get() - 1;
        unsafe { &mut *(self.ptr.add(last_idx)) }
    }

    /// Returns the first and all the rest of the elements of the slice.
    ///
    /// ```
    /// # use oom::NonEmptyMutSlice;
    /// let arr = &mut [10, 40, 30];
    /// let s = NonEmptyMutSlice::from_slice(arr);
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
    pub fn split_first_mut(&mut self) -> (&'a mut T, &'a mut [T]) {
        let ptr = self.ptr;
        let first = unsafe { &mut *ptr };
        let rest = unsafe {
            let ptr = ptr.add(1);
            let len = self.len().get() - 1;
            slice::from_raw_parts_mut(ptr, len)
        };
        (first, rest)
    }

    /// Returns the last and all the rest of the elements of the slice.
    ///
    /// ```
    /// # use oom::NonEmptyMutSlice;
    /// let arr = &mut [10, 40, 30];
    /// let s = NonEmptyMutSlice::from_slice(arr);
    /// assert_eq!(s.split_last(), (&30, &[10, 40][..]));
    /// ```
    pub fn split_last(&self) -> (&'a T, &'a [T]) {
        let ptr = self.ptr;
        let len = self.len().get() - 1;
        let rest = unsafe { slice::from_raw_parts(ptr, len) };
        let last = unsafe { &*(ptr.add(len)) };
        (last, rest)
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
    pub fn split_last_mut(&mut self) -> (&'a mut T, &'a mut [T]) {
        let ptr = self.ptr;
        let len = self.len().get() - 1;
        let rest = unsafe { slice::from_raw_parts_mut(ptr, len) };
        let last = unsafe { &mut *(ptr.add(len)) };
        (last, rest)
    }
}
