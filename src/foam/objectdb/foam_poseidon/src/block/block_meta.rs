#![allow(unused)]

use crate::{error::FP_BK_ILLEGAL_ARGUMENT, types::FPResult};

struct BlockRef {
    object_id: u32,
    offset: u64, // offset in .fp file.
    size: u32,
    checksum: u32,
}

/**
 * Unpack from bytes to block address.
 */
fn block_addr_unpack(raw_addr: &[u8], addr_size: usize) -> FPResult<()> {
    if addr_size == 0 {
        return Err(FP_BK_ILLEGAL_ARGUMENT);
    }

    _block_addr_unpack(raw_addr, addr_size)
}

fn _block_addr_unpack(raw_addr: &[u8], addr_size: usize) -> FPResult<()> {
    Ok(())
}