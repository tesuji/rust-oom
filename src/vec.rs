extern crate alloc;

use alloc::vec::Vec;
use core::cmp::Ordering;
use core::hint::unreachable_unchecked;
use core::mem::size_of;
use core::num::NonZeroUsize;

use crate::{NonEmptyMutSlice, NonEmptySlice};

/// A non-empty vector type, counterpart of `Vec<T>`.
pub struct NonEmptyVec<T: Sized> {
    inner: Vec<T>,
}

const _SIZE: () = {
    const FOO: [(); 1] = [()];
    const SIZE: usize = size_of::<NonEmptyVec<&str>>();
    #[cfg(target_pointer_width = "64")]
    let idx = !(SIZE == 24) as usize;
    #[cfg(target_pointer_width = "32")]
    let idx = !(SIZE == 12) as usize;
    FOO[idx]
};

const _BUILTIN_TRAITS: () = {
    impl<T: Clone> Clone for NonEmptyVec<T> {
        fn clone(&self) -> Self {
            Self::from_vec(self.to_vec())
        }
    }

    impl<T: Eq> Eq for NonEmptyVec<T> {}

    impl<T: PartialEq> PartialEq for NonEmptyVec<T> {
        fn eq(&self, other: &Self) -> bool {
            self.as_slice().eq(other.as_slice())
        }
    }

    impl<T: Ord> Ord for NonEmptyVec<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.as_slice().cmp(other.as_slice())
        }
    }

    impl<T: PartialOrd> PartialOrd for NonEmptyVec<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.as_slice().partial_cmp(other.as_slice())
        }
    }

    impl<T> AsRef<[T]> for NonEmptyVec<T> {
        fn as_ref(&self) -> &[T] {
            self.as_slice()
        }
    }
};

impl<T: Sized> NonEmptyVec<T> {
    /// Converts a `Vec<T>` into a `NonEmptyVec`.
    ///
    /// # Panics
    ///
    /// This function will panic if passed `Vec` is empty.
    pub fn from_vec(vec: Vec<T>) -> Self {
        match Self::from_vec_checked(vec) {
            Ok(v) => v,
            Err(_) => panic!("vec shouldn't be empty"),
        }
    }

    /// Converts a `Vec<T>` into a `NonEmptyVec`.
    /// Returns passed `Vec` if it is empty.
    pub fn from_vec_checked(vec: Vec<T>) -> Result<Self, Vec<T>> {
        if vec.is_empty() {
            return Err(vec);
        }
        Ok(Self { inner: vec })
    }

    /// Returns a raw pointer to the vector's buffer.
    pub fn as_ptr(&self) -> *const T {
        self.inner.as_ptr()
    }

    /// Returns an unsafe mutable pointer to the vector's buffer.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.inner.as_mut_ptr()
    }

    /// Returns a non-empty slice from this vec.
    pub fn as_nonempty_slice(&self) -> NonEmptySlice<'_, T> {
        NonEmptySlice { inner: &self.inner }
    }

    /// Returns a non-empty mutable slice from this vec.
    pub fn as_nonempty_mut_slice(&mut self) -> NonEmptyMutSlice<'_, T> {
        NonEmptyMutSlice {
            inner: &mut self.inner,
        }
    }

    /// Extracts a slice containing the entire vector.
    pub fn as_slice(&self) -> &[T] {
        self.inner.as_slice()
    }

    /// Extracts a mutable slice of the entire vector.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.inner.as_mut_slice()
    }

    /// Returns the number of elements in the vector.
    pub fn len(&self) -> NonZeroUsize {
        unsafe { NonZeroUsize::new_unchecked(self.inner.len()) }
    }

    /// Always returns `false` because the vector is non-empty.
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Returns the number of elements the vector can hold without reallocating.
    pub fn capacity(&self) -> NonZeroUsize {
        unsafe { NonZeroUsize::new_unchecked(self.inner.capacity()) }
    }

    /// Converts `self` into a vector without clones or allocations.
    pub fn into_vec(self) -> Vec<T> {
        self.inner
    }

    /// Copies `self` into a new `Vec`.
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.as_slice().to_vec()
    }

    /// Returns the first element of the slice.
    pub fn first(&self) -> &T {
        match self.as_slice() {
            [first, ..] => first,
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns a mutable pointer to the first element of the slice.
    pub fn first_mut(&mut self) -> &mut T {
        match self.as_mut_slice() {
            [first, ..] => first,
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the last element of the slice.
    pub fn last(&self) -> &T {
        match self.as_slice() {
            [.., last] => last,
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the last element of the slice.
    pub fn last_mut(&mut self) -> &mut T {
        match self.as_mut_slice() {
            [.., last] => last,
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the first and all the rest of the elements of the slice.
    pub fn split_first(&self) -> (&T, &[T]) {
        match self.as_slice() {
            [first, rest @ ..] => (first, rest),
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the first and all the rest of the elements of the slice.
    pub fn split_first_mut(&mut self) -> (&mut T, &mut [T]) {
        match self.as_mut_slice() {
            [first, rest @ ..] => (first, rest),
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the last and all the rest of the elements of the slice.
    pub fn split_last(&self) -> (&T, &[T]) {
        match self.as_slice() {
            [rest @ .., last] => (last, rest),
            [] => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the last and all the rest of the elements of the slice.
    pub fn split_last_mut(&mut self) -> (&mut T, &mut [T]) {
        match self.as_mut_slice() {
            [rest @ .., last] => (last, rest),
            [] => unsafe { unreachable_unchecked() },
        }
    }
}
