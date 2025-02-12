#![allow(unused)]

use crate::{error::FP_NO_IMPL, internal::FPResult};

use super::{handle::BlkHandle, meta::BlkAddr, BlkItem};

/**
 * Block manager.
 */
struct BlkMgr<MH> {
    handle: BlkHandle<MH>,
}

impl <MH> BlkMgr <MH> {
    /**
     * __wt_bm_read
     * 1. convert addr to BlkAddr
     * 2. read Block from handle.
     */
    fn read(&self, addr: &[u8]) -> FPResult<BlkItem<MH>> {
        let addr = BlkAddr::new(addr, 4*1024);

        //FEAT TODO: multi block handles.
        self.handle.read(addr)
    }
}