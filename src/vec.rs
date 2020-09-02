extern crate alloc;

use alloc::vec::Vec;
use core::mem::ManuallyDrop;
use core::num::NonZeroUsize;

use crate::NonEmptyMutSlice;

pub struct NonEmptyVec<'a, T: Sized> {
    inner: NonEmptyMutSlice<'a, T>,
    cap: NonZeroUsize,
}

impl<'a, T: Sized> NonEmptyVec<'a, T> {
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
        let len = vec.len();
        let cap = unsafe { NonZeroUsize::new_unchecked(vec.capacity()) };
        Ok(Self {
            inner: unsafe { NonEmptyMutSlice::from_raw_parts_mut(ptr, len) },
            cap,
        })
    }

    pub fn as_ptr(&self) -> *const T {
        self.inner.as_ptr()
    }

    pub fn as_mut_ptr(&self) -> *mut T {
        self.inner.as_mut_ptr()
    }

    pub fn as_slice(&self) -> &'a [T] {
        self.inner.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &'a mut [T] {
        self.inner.as_mut_slice()
    }

    pub fn len(&self) -> NonZeroUsize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn capacity(&self) -> NonZeroUsize {
        self.cap
    }

    pub fn into_vec(self) -> Vec<T> {
        let ptr = self.inner.as_mut_ptr();
        let len = self.inner.len().get();
        let cap = self.cap.get();
        unsafe { Vec::from_raw_parts(ptr, len, cap) }
    }

    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.inner.as_slice().to_vec()
    }
}
