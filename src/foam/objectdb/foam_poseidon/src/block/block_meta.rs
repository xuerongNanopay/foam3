#![allow(unused)]

use crate::{error::*, types::FPResult, util::compaction::varint::VarintDecodeIterator, FP_ASSERT_NOT_NONE};

static FP_BLOCK_ADDR_INDICATOR: u8 = 0x01;

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
    let mut iter = VarintDecodeIterator::new(bytes);
    
    let o = FP_ASSERT_NOT_NONE!(iter.next(), FP_BK_ILLEGAL_ARGUMENT); /* offset */
    let s = FP_ASSERT_NOT_NONE!(iter.next(), FP_BK_ILLEGAL_ARGUMENT); /* size */
    let c = FP_ASSERT_NOT_NONE!(iter.next(), FP_BK_ILLEGAL_ARGUMENT); /* checksum */

    // For checkpoint.
    let mut i: u64;
    if addr_size != 0 && iter.maybe_next() {
        let cur = iter.cur().unwrap();
        if FP_BIT_IS_SET!(cur, FP_BLOCK_ADDR_INDICATOR) {
            i = FP_ASSERT_NOT_NONE!(iter.next(), FP_BK_ILLEGAL_ARGUMENT); 
        }
    }
    Ok(())
}