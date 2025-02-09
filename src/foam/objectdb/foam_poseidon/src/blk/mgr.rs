#![allow(unused)]

use crate::{error::FP_NO_IMPL, internal::FPResult};

use super::{handle::BlkHandle, meta::BlkAddr, BlkItem};

/**
 * Block manager.
 */
struct BlkMgr {
    handle: BlkHandle,
}

impl BlkMgr {
    fn read(&self, addr: &[u8]) ->FPResult<BlkItem> {
        let addr = BlkAddr::new(addr, 4*1024);

        //NEED TODO: multi block.


        Err(FP_NO_IMPL)
    }
}