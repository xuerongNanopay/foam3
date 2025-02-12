#![allow(unused)]

use crate::{error::*, internal::FPResult, util::compaction::varint::{self, VarintDecodeIterator}, FP_ASSERT_NOT_NONE};

use super::{handle::BlkHandle, BlockRef};

static FP_BLOCK_ADDR_INDICATOR: u8 = 0x01;

/**
 * Unpack from bytes to block address.
 */
pub(crate) fn block_addr_unpack(block_handle: &BlkHandle, bytes: &[u8], addr_size: usize) -> FPResult<BlockRef> {
    let mut iter = VarintDecodeIterator::new(bytes);
    
    let offset = FP_ASSERT_NOT_NONE!(iter.next(), FP_BK_ILLEGAL_ARGUMENT); /* offset */
    let size = FP_ASSERT_NOT_NONE!(iter.next(), FP_BK_ILLEGAL_ARGUMENT); /* size */
    let checksum = FP_ASSERT_NOT_NONE!(iter.next(), FP_BK_ILLEGAL_ARGUMENT); /* checksum */

    // For checkpoint.
    let mut source_id = 0u64;
    if addr_size != 0 && iter.maybe_next() {
        let cur = iter.cur().unwrap();
        if FP_BIT_IST!(cur, FP_BLOCK_ADDR_INDICATOR) {
            source_id = FP_ASSERT_NOT_NONE!(iter.next(), FP_BK_ILLEGAL_ARGUMENT); 
        }
    }

    //TODO: assert the bytes is consume.

    if size == 0 {
        return Ok(BlockRef::default());
    } else {
        return Ok(BlockRef{
            source_id: source_id as u32,
            offset: (offset + 1) * block_handle.allocation_size,
            size: size * block_handle.allocation_size,
            checksum: checksum as u32,
        })
    }
}

pub(crate) fn block_addr_pack(bh: &BlkHandle, bytes: &mut [u8], block_ref: &BlockRef) -> FPResult<()> {
    let mut offset = 0u64;
    let mut size = 0u64;
    let mut checksum = 0u64;
    let mut source_id = 0u64;

    if block_ref.size != 0 {
        offset    = (block_ref.offset / bh.allocation_size) - 1 as u64;
        size      = block_ref.size / bh.allocation_size as u64;
        checksum  = block_ref.checksum as u64;
        source_id = block_ref.source_id as u64;
    }

    let cur = varint::encode_uint_inline(offset, bytes)?;
    let cur = varint::encode_uint_inline(size, &mut bytes[cur..])?;
    let mut cur = varint::encode_uint_inline(checksum, &mut bytes[cur..])?;

    if source_id != 0 {
        bytes[cur] = FP_BLOCK_ADDR_INDICATOR;
        cur += 1;
        varint::encode_uint_inline(source_id, &mut bytes[cur..])?;
    }

    Ok(())
}