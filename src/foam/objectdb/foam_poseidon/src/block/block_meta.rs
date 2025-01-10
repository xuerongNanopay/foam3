#![allow(unused)]

use crate::{error::*, types::FPResult, util::compaction::varint::VarintDecodeIterator, FP_ASSERT_NOT_NONE};

struct BlockRef {
    object_id: u32,
    offset: u64, // offset in .fp file.
    size: u32,
    checksum: u32,
}

/**
 * Unpack from bytes to block address.
 */
fn block_addr_unpack(bytes: &[u8], addr_size: usize) -> FPResult<()> {
    if addr_size == 0 {
        return Err(FP_BK_ILLEGAL_ARGUMENT);
    }

    _block_addr_unpack(bytes, addr_size)
}

fn _block_addr_unpack(bytes: &[u8], addr_size: usize) -> FPResult<()> {
    let mut iter = VarintDecodeIterator::new_with_limit(bytes, addr_size);
    
    let o = FP_ASSERT_NOT_NONE!(iter.next(), FP_BK_ILLEGAL_ARGUMENT); /* offset */
    let s = FP_ASSERT_NOT_NONE!(iter.next(), FP_BK_ILLEGAL_ARGUMENT); /* size */
    let c = FP_ASSERT_NOT_NONE!(iter.next(), FP_BK_ILLEGAL_ARGUMENT); /* checksum */

    // if addr_size != 0 && 
    Ok(())
}