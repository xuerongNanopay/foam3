#![allow(unused)]

use crate::{error::*, types::FPResult, util::compaction::varint::VarintDecodeIterator, FP_ASSERT_NOT_NONE};

static FP_BLOCK_ADDR_INDICATOR: u8 = 0x01;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
struct BlockRef {
    object_id: u32,
    offset: u64, // offset in .fp file.
    size: u32,
    checksum: u32,
}

/**
 * Unpack from bytes to block address.
 */
fn block_addr_unpack(bytes: &[u8], addr_size: usize) -> FPResult<BlockRef> {
    if addr_size == 0 {
        return Err(FP_BK_ILLEGAL_ARGUMENT);
    }

    _block_addr_unpack(bytes, addr_size)
}

fn _block_addr_unpack(bytes: &[u8], addr_size: usize) -> FPResult<BlockRef> {
    let mut iter = VarintDecodeIterator::new(bytes);
    
    let offset = FP_ASSERT_NOT_NONE!(iter.next(), FP_BK_ILLEGAL_ARGUMENT); /* offset */
    let size = FP_ASSERT_NOT_NONE!(iter.next(), FP_BK_ILLEGAL_ARGUMENT); /* size */
    let checksum = FP_ASSERT_NOT_NONE!(iter.next(), FP_BK_ILLEGAL_ARGUMENT); /* checksum */

    // For checkpoint.
    let mut object_id = 0u64;
    if addr_size != 0 && iter.maybe_next() {
        let cur = iter.cur().unwrap();
        if FP_BIT_IS_SET!(cur, FP_BLOCK_ADDR_INDICATOR) {
            object_id = FP_ASSERT_NOT_NONE!(iter.next(), FP_BK_ILLEGAL_ARGUMENT); 
        }
    }

    //TODO: assert the bytes is consume.

    if size == 0 {
        return Ok(BlockRef::default());
    } else {
        return Ok(BlockRef{
            object_id: object_id as u32,
            offset,
            size: size as u32,
            checksum: checksum as u32,
        })
    }
}