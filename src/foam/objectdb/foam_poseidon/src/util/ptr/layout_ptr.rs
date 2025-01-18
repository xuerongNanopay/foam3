#![allow(unused)]

use std::{alloc::Layout, ops::{Deref, DerefMut}};

pub struct LayoutPtr<T> {
    layout: Layout,
    ptr: *mut T,
}

impl <T> LayoutPtr<T> {
    pub fn new(layout: Layout, ptr: *mut T) -> LayoutPtr<T> {
        LayoutPtr{
            layout,
            ptr,
        }
    }
}

impl<T> Deref for LayoutPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.ptr as *const T) }
    }
}

impl<T> DerefMut for LayoutPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.ptr as *mut T) }
    }
}

impl<T>  Drop for LayoutPtr<T> {
    fn drop(&mut self) {
        FP_DEALLOC!(self.ptr, self.layout);
        self.ptr = std::ptr::null_mut();
    }
}