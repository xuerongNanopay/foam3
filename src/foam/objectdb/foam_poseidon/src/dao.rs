#![allow(unused)]

use crate::{error::FP_NO_IMPL, internal::FPResult};

/**
 * Data access object interface.
 */
pub(crate) trait DAO {
    fn free(&self) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }
    fn write(&mut self, buf: &[u8]) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }

    fn write_size(&self, len: usize) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }
}