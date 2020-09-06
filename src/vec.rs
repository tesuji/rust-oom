extern crate alloc;

use alloc::vec::Vec;
use core::cmp::Ordering;
use core::mem::size_of;
use core::num::NonZeroUsize;
use core::ops::{Deref, DerefMut};

use crate::NonEmptySlice;

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

    impl<T> Deref for NonEmptyVec<T> {
        type Target = NonEmptySlice<T>;
        fn deref(&self) -> &Self::Target {
            unsafe {
                &*(&self.inner[..] as *const [T] as *const NonEmptySlice<T>)
            }
        }
    }

    impl<T> DerefMut for NonEmptyVec<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                &mut *(&mut self.inner[..] as *mut [T] as *mut NonEmptySlice<T>)
            }
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
}
