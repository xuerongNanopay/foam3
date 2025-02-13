#![allow(unused)]

use crate::{btree::page::{self, FP_BTREE_PAGE_COMPRESSED}, error::FP_NO_IMPL, internal::FPResult};

use super::{compress::Compressor, handle::BlkHandle, meta::BlkAddr, BlkItem, PageHeader};

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
    compressor: Option<Box<dyn Compressor>>
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
        let mut blk_item: BlkItem;

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

            let page_header = PageHeader::from(&blk_item.mem[..]);

            //FEAT TODO: decrypt.

            //MUST TODO: store in cash

            if FP_BIT_IST!(page_header.flags, FP_BTREE_PAGE_COMPRESSED) {
                match &self.compressor {
                    None => panic!("Miss compressor for compressed page."),
                    Some(compressor) => {
                        let mut de_raw = Vec::<u8>::with_capacity(page_header.mem_size as usize);
                        //TODO: don't hard code.
                        let skip = 64usize;
                        de_raw[..skip].clone_from_slice(&blk_item.mem[..skip]);
                        //TODO: decompress.

                    }
                }
            }
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