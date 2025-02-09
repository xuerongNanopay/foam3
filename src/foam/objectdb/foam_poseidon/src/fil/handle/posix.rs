#![allow(unused)]

use crate::{error::FP_NO_IMPL, internal::FPResult};

use super::{FilBuf, FilHandle};

struct PosixFilHandle {

}

impl FilHandle for PosixFilHandle {
    fn read(&self, offset: u64, len: u64) -> FPResult<FilBuf> {
        //TODO: monitor.
        Err(FP_NO_IMPL)
    }
}