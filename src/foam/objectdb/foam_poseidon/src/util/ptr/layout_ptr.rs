#![allow(unused)]

use std::{alloc::Layout, ops::{Deref, DerefMut}};

#[repr(C)]
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
    }
}

#[cfg(test)]
mod tests {
    use std::mem::ManuallyDrop;

    use super::*;

    struct MyStruct {
        data: String,
    }

    impl Drop for MyStruct {
        fn drop(&mut self) {
            println!("Dropping MyStruct with data: {}", self.data);
        }
    }

    
    #[test]
    fn test_manualdrop() {
        let mut my_struct = ManuallyDrop::new(MyStruct {
            data: String::from("Hello, manual drop!"),
        });
    
        // Access fields safely
        println!("Data: {}", my_struct.data);
    
        // Manually drop the struct
        unsafe {
            ManuallyDrop::drop(&mut my_struct);
        }

        println!("end {}", my_struct.data);

    }
}