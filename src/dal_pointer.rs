// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

pub use std::rc::Rc;
use std::{cell::RefCell, rc::Weak};

#[derive(Debug)]
enum PtrWrapper<T> {
    None,
    V(T),
}

pub struct DalPtr<T> {
    ptr: PtrWrapper<Rc<RefCell<T>>>,
}

impl<T> DalPtr<T> {
    pub fn new(t: T) -> Self {
        Self {
            ptr: PtrWrapper::V(Rc::new(RefCell::new(t))),
        }
    }

    pub fn null() -> Self {
        Self {
            ptr: PtrWrapper::None,
        }
    }

    pub fn is_null(&self) -> bool {
        match &self.ptr {
            PtrWrapper::None => true,
            _ => false,
        }
    }

    pub fn is_some(&self) -> bool {
        !self.is_null()
    }

    pub fn borrow(&self) -> std::cell::Ref<T> {
        self.try_borrow().unwrap()
    }

    pub fn try_borrow(&self) -> Result<std::cell::Ref<T>, std::cell::BorrowError> {
        match &self.ptr {
            PtrWrapper::None => panic!("Cannot borrow a None pointer"),
            PtrWrapper::V(v) => v.try_borrow(),
        }
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<T> {
        self.try_borrow_mut().unwrap()
    }

    pub fn try_borrow_mut(&self) -> Result<std::cell::RefMut<T>, std::cell::BorrowMutError> {
        match &self.ptr {
            PtrWrapper::None => panic!("Cannot borrow a None pointer"),
            PtrWrapper::V(v) => v.try_borrow_mut(),
        }
    }

    pub fn downgrade(&self) -> DalWeakPtr<T> {
        match &self.ptr {
            PtrWrapper::None => panic!("Cannot downgrade a None pointer"),
            PtrWrapper::V(v) => DalWeakPtr {
                ptr: Rc::downgrade(v),
            },
        }
    }
}

impl<T> Clone for DalPtr<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: match &self.ptr {
                PtrWrapper::None => PtrWrapper::None,
                PtrWrapper::V(v) => PtrWrapper::V(Rc::clone(v)),
            },
        }
    }
}

#[derive(Debug)]
pub struct DalWeakPtr<T> {
    ptr: Weak<RefCell<T>>,
}

impl<T> DalWeakPtr<T> {
    pub fn new() -> Self {
        Self { ptr: Weak::new() }
    }

    pub fn upgrade(&self) -> Option<DalPtr<T>> {
        match self.ptr.upgrade() {
            None => None,
            Some(v) => Some(DalPtr {
                ptr: PtrWrapper::V(v),
            }),
        }
    }
}

impl<T> Clone for DalWeakPtr<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: Weak::clone(&self.ptr),
        }
    }
}
