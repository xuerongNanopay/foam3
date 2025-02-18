#![allow(unused)]

use crate::{error::FP_NO_IMPL, internal::FPResult};

/**
 * Block manager that manager block access for btree.
 */
trait BlockManager {
    fn close(&mut self) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }

    fn read(&self, offset: u64, size: u64) -> FPResult<Vec<u8>> {
        Err(FP_NO_IMPL)
    }

    fn write(&mut self, data: &[u8]) -> FPResult<()> {
        Err(FP_NO_IMPL)
    } 
}