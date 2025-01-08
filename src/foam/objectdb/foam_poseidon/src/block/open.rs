#![allow(unused)]

use manager::BlockManager;

use super::*;
use crate::error::{FP_BK_DATA_CORRUPTION, FP_BK_INVALID_MAGIC, FP_BK_INVALID_MAJOR, FP_BK_INVALID_MINOR, FP_IO_BROKEN_PIPE, FP_IO_UNEXPECTED_EOF};
use crate::misc::{FP_BLOCK_MAGIC, FP_BLOCK_MAJOR, FP_BLOCK_MINOR};
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
fn open_block(
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

    //TODO: cash block in somewhere, we can reuse later.

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
    let mut new_block = Arc::new(Block {
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
    });
    FP_ASSERT_FP_ERR!(read_meta(new_block.clone(), allocation_size));

    Ok(new_block)

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

    // Create new BlockHeader.
    let mut block_header = REINTERPRET_CAST_BUF_MUT!(buf, BlockHeader);

    // block_header.maybe_convert_endian();
    
    let save_checksum = block_header.checksum;
    let real_checksum = if cfg!(target_endian = "big") { BIT_REVERSE_32!(block_header.checksum) } else { block_header.checksum };

    //TODO: checksum verify.
    block_header.checksum = 0;

    block_header.checksum = save_checksum;

    block_header.maybe_convert_endian();

    if block_header.magic != FP_BLOCK_MAGIC {
        return Err(FP_BK_INVALID_MAGIC)
    }

    if block_header.major > FP_BLOCK_MAJOR {
        return Err(FP_BK_INVALID_MAJOR)
    }

    if block_header.major == FP_BLOCK_MAJOR && block_header.minor > FP_BLOCK_MINOR {
        return Err(FP_BK_INVALID_MINOR)
    }

    //see: block_open.c line: 400 func: __desc_read
    Ok(())
}

