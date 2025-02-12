#![allow(unused)]

use crate::{error::FP_NO_IMPL, internal::FPResult};

use super::{handle::BlkHandle, meta::BlkAddr, BlkItem, PageHeader};

#[derive(Default, Clone, Copy)]
enum BlkpoolType {
    #[default]
    Unconfigured,
    Dram,
    Nvram
}

struct BlkpoolItem {

}

/**
 * Each db file has its own block pool.
 */
struct Blkpool {
    blk_size: u32,
    blk_handle: BlkHandle,
}

impl  Blkpool  {

    /**
     * Read block.
     * __wt_blkcache_read
     * by: __page_read.
     * 1. Checks if the block exists in memory
     * 2. Reads from the block manager if not found in cache
     * 3. Handles decryption (if the block is encrypted). TODO
     * 4. Handles decompression (if the block is compressed). TODO
     * 5. Verifies the block if required.
     * 6. Stores the block in the block cache if applicable.
     */
    fn read(
        &self,
        addr: &[u8]
    ) -> FPResult<()> {

        let addr = BlkAddr::new(addr, self.blk_size);
        let blk_item: BlkItem;

        'read_blk: loop {
            //MUST TODO
            // Try read from pool.
            match self.read_from_pool(addr)? {
                Some(item) => { 
                    blk_item = item;
                    break 'read_blk; 
                },
                None => {},
            };

            //FEAT TODO: matrix
            blk_item = self.blkpool_read_blk(addr)?;

            let page_header = PageHeader::from(&blk_item.raw[..]);
            break 'read_blk;
        }
        Err(FP_NO_IMPL)
    }

    /**
     * Read block from pool.
     */
    fn read_from_pool(&self, addr: BlkAddr) -> FPResult<Option<BlkItem>> {
        Err(FP_NO_IMPL)
    }

    /**
     * Read block form file.
     * __bm_read -> __wt_bm_read -> __wti_block_read_off -> __wt_read -> fh_read
     */
    fn blkpool_read_blk(&self, addr: BlkAddr) -> FPResult<BlkItem> {
        self.blk_handle.read(addr)
    }

}