use std::{collections::HashMap, sync::{Arc, RwLock, RwLockWriteGuard}, usize};

use crate::{
    error::FP_BK_ILLEGAL_CHECKSUM, os::fil::{
        FPFileSystem, 
        FileHandle, 
        FileSystem, 
        FileType, 
        FP_FS_OPEN_CREATE, 
        FP_FS_OPEN_DURABLE, 
        FP_FS_OPEN_EXCLUSIVE
    }, types::{
        FPConcurrentHashMap, 
        FPResult
    }, FP_CHECKSUM_EQ, FP_LOG_ERR, FP_STATS_INCR
};

use super::{block_handle::{self, file_header_write, BlockHandle}, block_ref, BlockHeader, BlockRef, PageHeader};

//TODO: drop file object from directory.(block_open.c line28)

/**
 * Block manager, reference to a block(block reference to a file).
 */
pub(crate) struct BlockManager {
    block_handle: Arc<BlockHandle>,

    is_multi: bool, /* TODO: allow store block into mutli handle */
}

/**
 * Create a file and write meta data to it.
 */
fn create(file_system: Arc<FPFileSystem>, filename: &str, alloc_size: u32) -> FPResult<()> {
    let fh = file_system.open(filename, FileType::Data, FP_FS_OPEN_CREATE | FP_FS_OPEN_DURABLE | FP_FS_OPEN_EXCLUSIVE)?;
    file_header_write(fh.clone(), alloc_size)?;
    fh.sync()?;
    file_system.close_fh(&fh.name)?;
    Ok(())
}

/**
 * Drop a file.
 */
fn drop() {

}

/**
 * Read a block base on block reference(BlockRef).
 */
fn read(block_manager: Arc<BlockManager>, raw_addr: &[u8], addr_size: usize) -> FPResult<()> {

    let bh = block_manager.block_handle.clone();

    let br = block_ref::block_addr_unpack(&bh, raw_addr, addr_size)?;
    
    FP_STATS_INCR!(block_read);
    FP_STATS_INCR!(block_size, br.size);

    read_offset_from_bh(&bh, &br);

    //TODO: read from block handle.
    //TODO: discard.

    Ok(())
}

/**
 * Read a block into a buffer.
 * TODO: retry logic and checksum verify.
 */
fn read_offset_from_bh(block_handle: &BlockHandle, block_ref: &BlockRef) -> FPResult<Vec<u8>> {
    
    if block_ref.size < block_handle.allocation_size {
        FP_LOG_ERR!("block handle size {} is less than allocation size {}.", block_handle.size, block_ref.size);
    }

    //TODO: retry.

    let (mut r_buf, r_size) = block_handle.file_handle.read_exact(block_ref.offset, block_ref.size)?;
    let mut block_header_in_buf = FP_REINTERPRET_CAST_BUF_MUT!(r_buf, BlockHeader, SIZE_OF!(PageHeader));
    let mut block_header= *block_header_in_buf;
    block_header.maybe_convert_endian();

    // Valify check sum
    if block_ref.checksum == block_header.checksum {
        block_header_in_buf.checksum = 0;
        if FP_CHECKSUM_EQ!(&r_buf, size, block_header.checksum) {
            return Ok(r_buf);
        }
    }

    Err(FP_BK_ILLEGAL_CHECKSUM)
}

// pub struct BlockManager {
//     blocks: FPConcurrentHashMap<String, Arc<Block>>,
// }

// impl BlockManager {
//     pub(crate) fn get_block(&self, filename: &str) -> Option<Arc<Block>> {
//         let blocks = self.blocks.read().unwrap();
//         match blocks.get(filename) {
//             Some(v) => Some(Arc::clone(v)),
//             None => None
//         }
//     }
//     pub(crate) fn insert_block(&self, filename: &str, block: Arc<Block>) {
//         let mut blocks = self.blocks.write().unwrap();
//         blocks.insert(String::from(filename), block);
//     }

//     pub(crate) fn get_block_or_default<F>(&self, filename: &str, init_f: F) -> FPResult<Arc<Block>>
//     where F: Fn() -> FPResult<Arc<Block>>
//     {
//         if let Some(o) = self.get_block(filename) {
//             return Ok(o);
//         }

//         let mut blocks = self.blocks.write().unwrap();
//         let new_block = init_f()?;
//         let ret = new_block.clone();
//         blocks.insert(String::from(filename), new_block);
//         Ok(ret)
//     }
// }