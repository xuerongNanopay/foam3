#![allow(unused)]

use crate::{error::FP_NO_IMPL, internal::FPResult};

use super::BlkItem;

/**
 * Block manager.
 */
struct BlkMgr {

}

impl BlkMgr {
    fn read(&self, addr: &[u8]) ->FPResult<BlkItem> {
        Err(FP_NO_IMPL)
    }
}