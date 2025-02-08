#![allow(unused)]

use crate::{error::FP_NO_IMPL, internal::FPResult};

/**
 * Read block.
 * 
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

    Err(FP_NO_IMPL)
}

/**
 * Try read block from pool.
 */
fn blkpool_pool_read(
    addr: &[u8]
) -> FPResult<Option<()>> {

    Err(FP_NO_IMPL)
}