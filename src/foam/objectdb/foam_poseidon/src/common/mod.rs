#![allow(unused)]

pub(crate) use self::arc_bytes::{ArcBytes};

pub mod file;
mod arc_bytes;

pub(crate) trait Length {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}