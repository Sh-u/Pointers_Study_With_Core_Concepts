//! Basic implementation of Rc shared pointer

use crate::cell::MyCell;
use std::{marker::PhantomData, ops::Deref, ptr::NonNull};

/// # Required to use PhantomData
/// It tells the compiler that when you drop the Rc
/// an RcInner<T> might be dropped, and you need to check that.
/// In other words, it marks the RcInner<T> as owned by the Rc.

/// # Info
/// Doesn't impl Sync and Send because
/// if you would send the Rc to a different thread then both
/// of the threads would possibly drop it at the same time,
/// which is not okay (cell is not thread safe).
///
/// Never provides mutability.
///
/// Allows you to have multiple shared refs.
///
/// Gets deallocated when the last one goes away.
/// # Common Usage
/// Useful in data structures where you might have
/// one element be present in multiple places
/// e.g when you have something like config
/// and you dont want to make many copies of it.

pub struct MyRc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

/// # Struct to store T value and ref count of the Rc
pub struct RcInner<T> {
    value: T,
    ref_count: MyCell<usize>,
}

impl<T> MyRc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            ref_count: MyCell::new(1),
            value: value,
        });

        MyRc {
            // SAFE because box cannot give us a nullptr
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let current_refs = inner.ref_count.get();
        inner.ref_count.set(current_refs + 1);
        MyRc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        //SAFE because inner struct is deallocated only when
        //the last Rc goes away
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let current_refs = inner.ref_count.get();
        if current_refs == 1 {
            drop(inner);
            //SAFE because we are keeping the ref count
            //and will drop the pointer when the last rc goes away
            unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            inner.ref_count.set(current_refs - 1)
        }
    }
}
