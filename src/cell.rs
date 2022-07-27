//! # A basic custom implementation of a cell mutable container
//! With most essential info about it

/// # Rule 1
/// You cannot get a ref to the value inside the cell,
/// therefore it's always safe to mut it
/// # Rule 2
/// Doesn't impl Sync, so if you have a ref to a cell
/// you cannot give it away to a different thread
///
/// # Info
/// Guarantees at compile time
///
/// # Common Usage
///
///
///
/// Used for small copy types
///
/// For small(er) values like numbers/flags that need to be mutated from multiple places
/// e.g Thread Locals
///
/// When you want to have multiple shared ref to a thing
///
///
use std::{cell::UnsafeCell, ptr};

/// # Required to wrap T in UnsafeCell
/// because you are never allowed to cast a shared ref to an exclusive ref
/// in other way than by going through the unsafe cell. It's the only way to implement interior mutability
#[derive(Debug)]
pub struct MyCell<T> {
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

// implied by UnsafeCell
// impl<T> !Sync for MyCell<T> {}

pub mod utils {
    use std::cell::Cell;

    pub fn cell_operations() {
        let c1 = Cell::new(5);
        let c2 = Cell::new(10);

        println!("initial c1: {:#?}", c1);

        println!("initial c2: {:#?}", c2);

        c1.set(1);
        c2.set(2);

        println!("set c1: {:#?}", c1);

        println!("set c2: {:#?}", c2);

        c1.swap(&c2);
        c2.swap(&c1);

        println!("swap c1: {:#?}", c1);

        println!("swap c2: {:#?}", c2);

        let old_val = c1.replace(10);
        let old_val2 = c2.replace(20);

        println!("replace old c1: {} to {:#?}", old_val, c1);

        println!("replace old c2: {} to {:#?}", old_val2, c2);

        let ten = c1.get();
        let twenty = c2.get();

        println!("get c1 value copy: {:#?}", ten);

        println!("get c2 value copy : {:#?}", twenty);
    }
}
