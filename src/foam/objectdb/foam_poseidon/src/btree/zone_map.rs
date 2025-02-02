#![allow(unused)]

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct ZMPage {
    pub(crate) start_ts: u64,
    pub(crate) stop_ts: u64,
}