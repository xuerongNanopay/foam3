#![allow(unused)]

use std::sync::atomic::{AtomicBool, Ordering};

pub struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }

    fn try_lock(&self) -> bool {
        self.locked.swap(true, Ordering::Acquire)
    }

    fn lock(&self) {
        while self.locked.swap(true, Ordering::Acquire) {
            //lock.
        }
    }

    fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}