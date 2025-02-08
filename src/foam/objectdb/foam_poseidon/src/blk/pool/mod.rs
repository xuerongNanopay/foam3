#![allow(unused)]

use crate::{error::FP_NO_IMPL, internal::FPResult};

#[derive(Default, Clone, Copy)]
enum BlkpoolType {
    #[default]
    Unconfigured,
    Dram,
    Nvram
}

struct Blkpool {
}

struct BlkpoolItem {

}

/**
 * Read block.
 * __wt_blkcache_read
 * 1. Checks if the block exists in memory
 * 2. Reads from the block manager if not found in cache
 * 3. Handles decryption (if the block is encrypted). TODO
 * 4. Handles decompression (if the block is compressed). TODO
 * 5. Verifies the block if required.
 * 6. Stores the block in the block cache if applicable.
 */
fn blkpool_read(
    addr: &[u8]
) -> FPResult<()> {

    'read_blk: loop {
        //MUST TODO
        // Try read from pool.
        match blkpool_read_pool(addr)? {
            Some(todo) => { break 'read_blk; },
            None => {},
        };

        blkpool_read_blk(addr);

        break 'read_blk;
    }
    Err(FP_NO_IMPL)
}

/**
 * Try read block from pool.
 */
fn blkpool_read_pool(
    addr: &[u8]
) -> FPResult<Option<()>> {

    Err(FP_NO_IMPL)
}

/**
 * Read block form file.
 * __bm_read -> __wt_bm_read -> __wti_block_read_off -> __wt_read -> fh_read
 */
fn blkpool_read_blk(
    addr: &[u8]
) -> FPResult<Option<()>> {

    Err(FP_NO_IMPL)
}