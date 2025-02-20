#![allow(unused)]

use std::ops::Deref;

pub(crate) use self::bytes::{ByteSlice};

pub mod file;
mod bytes;

pub(crate) trait HasLength {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: Deref<Target = [u8]>> HasLength for T {
    fn len(&self) -> usize {
        self.deref().len()
    }
}