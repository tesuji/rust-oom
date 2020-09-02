extern crate alloc;

use alloc::vec::Vec;
use core::cmp::Ordering;
use core::mem::size_of;
use core::mem::ManuallyDrop;
use core::num::NonZeroUsize;
use core::ptr;

use crate::{NonEmptyMutSlice, NonEmptySlice};

/// A non-empty vector type, counterpart of `Vec<T>`.
pub struct NonEmptyVec<T: Sized> {
    ptr: *mut T,
    len: NonZeroUsize,
    cap: NonZeroUsize,
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

    impl<T> Drop for NonEmptyVec<T> {
        fn drop(&mut self) {
            unsafe {
                // Same code path as `Vec::drop`.
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.ptr, self.len.get()))
            }
        }
    }
};

impl<'a, T: Sized> NonEmptyVec<T> {
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
        let mut vec = ManuallyDrop::new(vec);
        let ptr = vec.as_mut_ptr();
        let len = unsafe { NonZeroUsize::new_unchecked(vec.len()) };
        let cap = unsafe { NonZeroUsize::new_unchecked(vec.capacity()) };
        Ok(Self { ptr, len, cap })
    }

    /// Returns a raw pointer to the vector's buffer.
    pub fn as_ptr(&self) -> *const T {
        self.ptr as *const T
    }

    /// Returns an unsafe mutable pointer to the vector's buffer.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr
    }

    /// Returns a non-empty slice from this vec.
    pub fn as_nonempty_slice(&self) -> NonEmptySlice<'a, T> {
        let ptr = self.as_ptr();
        let len = self.len().get();
        unsafe { NonEmptySlice::from_raw_parts(ptr, len) }
    }

    /// Returns a non-empty mutable slice from this vec.
    pub fn as_nonempty_mut_slice(&mut self) -> NonEmptyMutSlice<'a, T> {
        let ptr = self.as_mut_ptr();
        let len = self.len().get();
        unsafe { NonEmptyMutSlice::from_raw_parts_mut(ptr, len) }
    }

    /// Extracts a slice containing the entire vector.
    pub fn as_slice(&self) -> &'a [T] {
        self.as_nonempty_slice().as_slice()
    }

    /// Extracts a mutable slice of the entire vector.
    pub fn as_mut_slice(&mut self) -> &'a mut [T] {
        self.as_nonempty_mut_slice().as_mut_slice()
    }

    /// Returns the number of elements in the vector.
    pub fn len(&self) -> NonZeroUsize {
        self.len
    }

    /// Always returns `false` because the vector is non-empty.
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Returns the number of elements the vector can hold without reallocating.
    pub fn capacity(&self) -> NonZeroUsize {
        self.cap
    }

    /// Converts `self` into a vector without clones or allocations.
    pub fn into_vec(self) -> Vec<T> {
        let ptr = self.ptr;
        let len = self.len.get();
        let cap = self.cap.get();
        unsafe { Vec::from_raw_parts(ptr, len, cap) }
    }

    /// Copies `self` into a new `Vec`.
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.as_slice().to_vec()
    }

    /// A shorthand for [`NonEmptyMutSlice::first`].
    pub fn first(&self) -> &'a T {
        self.as_nonempty_slice().first()
    }

    /// A shorthand for [`NonEmptyMutSlice::first_mut`].
    pub fn first_mut(&mut self) -> &'a mut T {
        self.as_nonempty_mut_slice().first_mut()
    }

    /// A shorthand for [`NonEmptyMutSlice::last`].
    pub fn last(&self) -> &'a T {
        self.as_nonempty_slice().last()
    }

    /// A shorthand for [`NonEmptyMutSlice::last_mut`].
    pub fn last_mut(&mut self) -> &'a mut T {
        self.as_nonempty_mut_slice().last_mut()
    }

    /// A shorthand for [`NonEmptyMutSlice::split_first`].
    pub fn split_first(&self) -> (&'a T, &'a [T]) {
        self.as_nonempty_slice().split_first()
    }

    /// A shorthand for [`NonEmptyMutSlice::split_first_mut`].
    pub fn split_first_mut(&mut self) -> (&'a mut T, &'a mut [T]) {
        self.as_nonempty_mut_slice().split_first_mut()
    }

    /// A shorthand for [`NonEmptyMutSlice::split_last`].
    pub fn split_last(&self) -> (&'a T, &'a [T]) {
        self.as_nonempty_slice().split_last()
    }

    /// A shorthand for [`NonEmptyMutSlice::split_last_mut`].
    pub fn split_last_mut(&mut self) -> (&'a mut T, &'a mut [T]) {
        self.as_nonempty_mut_slice().split_last_mut()
    }
}
