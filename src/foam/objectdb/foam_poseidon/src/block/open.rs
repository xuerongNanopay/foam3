#![allow(unused)]

use super::*;
use crate::error::{FP_BK_DATA_CORRUPTION, FP_IO_BROKEN_PIPE, FP_IO_UNEXPECTED_EOF};
use crate::os::fil::{self, AccessMode, FPFileSystem, FileSystem, FileType};
use crate::types::FPResult;
use crate::util::hash_city;
use crate::FP_ASSERT_FP_ERR;
use std::collections::LinkedList;
use std::sync::{Mutex, Arc};
use std::sync::atomic::{AtomicUsize, Ordering};

struct BlockOpenCfg {
    allocation_size: FPFileSize,
    alloc_first: bool,
    os_cache_max: usize,
    os_cache_dirty_max: usize,
    extend_len: usize,
    access_mode: fil::AccessMode,
}



/**
 * 1. find block from ctx.
 * 2. if no found, create a new one.
 */
fn block_open(
    block_manager: Arc<BlockManager>,
    file_system: Arc<FPFileSystem>,
    default_cfg: BlockOpenCfg,
    filename: &str, 
    object_id: u32,
    allocation_size: FPFileSize,
    readonly: bool,
    fixed: bool,
) -> FPResult<Arc<Block>> {
    let hash = hash_city::city_hash_64(filename, filename.len());
    let bucket = hash % 10; //TODO: bucket size should be a config

    if let Some(b) = block_manager.get_block(filename) {
        return Ok(b);
    }

    let mut blocks = block_manager.blocks.write().unwrap();

    let mut flags = 0u32;

    match default_cfg.access_mode {
        AccessMode::Random => flags = BIT_SET!(flags, fil::FP_FS_OPEN_ACCESS_RAND),
        AccessMode::Sequential => flags = BIT_SET!(flags, fil::FP_FS_OPEN_ACCESS_SEQ)
    }

    if fixed {
        flags = BIT_SET!(flags, fil::FP_FS_OPEN_FIXED);
    }

    if readonly {
        flags = BIT_SET!(flags, fil::FP_FS_OPEN_READONLY);
    }

    let file_handle = FP_ASSERT_FP_ERR!(file_system.open(filename, FileType::Data, flags));
    let fh = file_handle.clone();

    // construct new block.
    let mut new_block = Block {
        name: filename.to_string(),
        object_id,
        allocation_size: if allocation_size == 0 {
            default_cfg.allocation_size
        } else {
            allocation_size
        },
        alloc_first: default_cfg.alloc_first,
        os_cache_max: default_cfg.os_cache_max,
        os_cache_dirty_max: default_cfg.os_cache_dirty_max,
        extend_len: default_cfg.extend_len,

        readonly,
        size: FP_ASSERT_FP_ERR!(fh.size()),
        file_handle,
    };

    let b = Arc::new(new_block);
    let ret = Arc::clone(&b);

    read_meta(b.clone(), allocation_size);

    block_manager.insert_block(filename, b);
    Ok(ret)
}

/**
 * Read and verify Meta block.
 */
fn read_meta(block: Arc<Block>, allocation_size: FPFileSize) -> FPResult<()> {

    if block.size < allocation_size {
        return Err(FP_BK_DATA_CORRUPTION)
    }

    //TODO: Metrix for the read func.

    let (mut buf, len) = FP_ASSERT_FP_ERR!(block.file_handle.read(0, allocation_size));

    let block_header = REINTERPRET_CAST_BUF_MUT!(buf, BlockHeader);

    let saved_checksum = block_header.checksum;

    Ok(())
}