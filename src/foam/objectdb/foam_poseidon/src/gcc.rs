#![allow(unused)]

#[inline(always)]
pub(crate) fn full_barrier() {
    unsafe {
        core::arch::asm!("dmb ish", options(nostack, preserves_flags));
    }
}

#[inline(always)]
pub(crate) fn acquire_barrier() {
    unsafe {
        core::arch::asm!("dmb ishld", options(nostack, preserves_flags));
    }
}