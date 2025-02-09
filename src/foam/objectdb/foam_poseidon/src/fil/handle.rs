#![allow(unused)]

mod posix;
mod native;

use crate::{error::FP_NO_IMPL, internal::FPResult};

pub(crate) struct FilBuf {
    data: Vec<u8>,
    size: u64,
}

pub(crate) trait FilHandle {
    fn read(&self, offset: u64, len: u64) -> FPResult<FilBuf> {
        //TODO: monitor.
        Err(FP_NO_IMPL)
    }
}