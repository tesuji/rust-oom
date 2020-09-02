extern crate alloc;

use alloc::vec::Vec;
use core::mem::ManuallyDrop;
use core::num::NonZeroUsize;

use crate::NonEmptyMutSlice;

pub struct NonEmptyVec<'a, T: Sized> {
    inner: NonEmptyMutSlice<'a, T>,
    cap: NonZeroUsize,
}

impl<'a, T: Sized> NonEmptyMutSlice<'a, T> {
    /// Converts a `&T` into a `NonEmptyMutSlice`.
    pub fn from_vec_checked(v: Vec<T>) -> Result<Self, Vec<T>> {
        if v.is_empty() {
            return Err(v);
        }
        let v = ManuallyDrop::new(v);
        let ptr = v.as_mut_ptr();
        let len = v.len();
        let cap = v.capacity();
        Self {
            inner: unsafe { NonEmptyMutSlice::from_raw_parts_mut(ptr, len) },
            cap,
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

        let ptr = slice.as_mut_ptr();
        let len = unsafe { NonZeroUsize::new_unchecked(slice.len()) };
        Some(Self {
            ptr,
            len,
            lt: PhantomData,
        })
    }
}
