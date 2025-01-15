#![allow(unused)]

use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::UnsafeCell;

struct SpinLock<T: ?Sized> {
    flag: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    fn new(data: T) -> Self {
        Self {
            flag: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    fn lock(&self) -> SpinLockGuard<T> {
        // Busy-wait until the lock becomes available
        while self.flag.swap(true, Ordering::Acquire) {}
        SpinLockGuard { lock: self }
    }
}

struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<'a, T> std::ops::Deref for SpinLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> std::ops::DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.flag.store(false, Ordering::Release);
    }
}
