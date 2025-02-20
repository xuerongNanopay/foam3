#![allow(unused)]

use std::ops::Deref;

pub(crate) use self::arc_bytes::{ArcBytes};

pub mod file;
mod arc_bytes;

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