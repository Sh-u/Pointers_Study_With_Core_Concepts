//! # Basic implementation of a Cell mutable container
//! With most essential info about it
use std::{cell::UnsafeCell, ptr};

/// # Info
/// Doesn't impl Sync, so if you have a ref to a cell
/// you cannot give it away to a different thread.
///
/// You cannot get a ref to the value inside the cell,
/// therefore it's always safe to mut it.
///
/// Guarantees it at compile time.
///
/// # Common Usage
///
/// Used for small copy types.
///
/// For small(er) values like numbers/flags that need to be mutated from multiple places
/// e.g Thread Locals.
///
/// When you want to have multiple shared ref to a thing.
///
///
/// # Required to wrap T in UnsafeCell
/// Because you are never allowed to cast a shared ref to an exclusive ref
/// in other way than by going through the unsafe cell. It's the only way to implement interior mutability.
#[derive(Debug)]
pub struct MyCell<T> {
    // implied by UnsafeCell
    // impl<T> !Sync for MyCell<T>
    value: UnsafeCell<T>,
}

impl<T: Copy> MyCell<T> {
    pub fn new(value: T) -> Self {
        MyCell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }

    pub fn swap(&self, other: &Self) {
        unsafe {
            if ptr::eq(self, other) {
                return;
            }

            ptr::swap(self.value.get(), other.value.get());
        }
    }
}
