//! # Basic implementation of a RefCell mutable memory location
//! With most essential info about it

use crate::cell::MyCell;
use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

/// # Required to wrap value T in UnsafeCell
/// Because you are never allowed to cast a shared ref to an exclusive ref
/// in other way than by going through the unsafe cell.

/// # Wrapping our RefState in Cell
/// Will give us ability to mutate Enum's reference count through a shared reference

/// # Info
/// RefCell will enforce borrow rules at runtime.

/// # Common Usage
/// A fairly safe way to dynamically borrow data
/// e.g Node in a graph/tree.

pub struct MyRefCell<T> {
    value: UnsafeCell<T>,
    reference: MyCell<RefState>,
}

#[derive(Copy, Clone)]
pub enum RefState {
    Shared(usize),
    Unshared,
    Exclusive,
}

impl<T> MyRefCell<T> {
    pub fn new(value: T) -> Self {
        MyRefCell {
            value: UnsafeCell::new(value),
            reference: MyCell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.reference.get() {
            RefState::Unshared => {
                self.reference.set(RefState::Shared(1));
                // SAFE because there are no other refs set yet
                Some(Ref { refcell: self })

                // Some(unsafe { Ref { refcell: &*self } })
            }
            RefState::Shared(n) => {
                self.reference.set(RefState::Shared(n + 1));
                // SAFE because we can have multiple immutable borrows

                Some(Ref { refcell: self })

                // Some(unsafe { Ref { refcell: &*self } })
            }
            _ => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        match self.reference.get() {
            RefState::Unshared => {
                // SAFE because we made sure there a no other refs yet
                // we can only have one exclusive borrow
                self.reference.set(RefState::Exclusive);

                Some(RefMut { refcell: self })
            }

            _ => None,
        }
    }
}

/// # Ref type as output for borrow method
/// We need this type to handle decrementing shared ref count
/// after they go out of scope
pub struct Ref<'refcell, T> {
    refcell: &'refcell MyRefCell<T>,
}

/// # Ref type as output for borrow_mut method
/// We need this to handle decrementing exclusive ref count
/// after they go out of scope
pub struct RefMut<'refcell, T> {
    refcell: &'refcell MyRefCell<T>,
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        if let RefState::Shared(n) = self.refcell.reference.get() {
            if n > 1 {
                self.refcell.reference.set(RefState::Shared(n - 1));
            } else {
                self.refcell.reference.set(RefState::Unshared);
            }
        } else {
            unreachable!()
        }
    }
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFE because ref is only created when there are
        // no exclusive refs given out
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFE because ref is only created when there are
        // no exclusive refs given out
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFE because ref mut is only created when there are
        // no refs given out.
        // Also it enforces that there will not be any future refs given out
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        if let RefState::Exclusive = self.refcell.reference.get() {
            self.refcell.reference.set(RefState::Unshared);
        } else {
            unreachable!()
        }
    }
}
