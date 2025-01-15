#![allow(unused)]

use super::*;
use crate::error::*;
use crate::meta::*;
use crate::os::fil::{self, AccessMode, FPFileHandle, FPFileSystem, FileHandle, FileSystem, FileType};
use crate::types::{FPFileSize, FPResult};
use crate::util::hash_city;
use std::collections::LinkedList;
use std::sync::{Mutex, Arc};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/**
 * Block; reference a single file.
 * Not physical representation of page.
 */

pub(crate) struct BlockHandle {
    name: String,   /* Name */
    object_id: u32,

    pub file_handle: Arc<FPFileHandle>,  /* underline file handle */
    pub(crate) size: FPFileSize,         /* File size */
    extend_size: FPFileSize,             /* File extended size */
    extend_len: FPFileSize,              /* File extend chunk size */

    sync_on_checkpoint: bool,     /* fsync the handle after the next checkpoint */
    remote: bool,                 /* remove handler */
    readonly: bool,               /* underline file is read only */

    pub(crate) allocation_size: FPFileSize,
    alloc_first: AtomicBool,

    // os_cache: usize,              
    // os_cache_max: usize,
    // os_cache_dirty_max: usize,

    // block_header_size: u32,
}

impl BlockHandle {
}

struct BlockOpenCfg {
    allocation_size: FPFileSize,
    alloc_first: bool,
    os_cache_max: usize,
    os_cache_dirty_max: usize,
    extend_len: FPFileSize,
    access_mode: fil::AccessMode,
}



/**
 * 1. find block handle from ctx.
 * 2. if no found, create a new one.
 */
fn open(
    file_system: Arc<FPFileSystem>,
    default_cfg: BlockOpenCfg,
    filename: &str, 
    object_id: u32,
    allocation_size: FPFileSize,
    readonly: bool,
    fixed: bool,
) -> FPResult<Arc<BlockHandle>> {
    let hash = hash_city::city_hash_64(filename, filename.len());
    let bucket = hash % 10; //TODO: bucket size should be a config

    //TODO: cache block in somewhere, we should only keep one block for a file.

    let mut flags = 0u32;

    match default_cfg.access_mode {
        AccessMode::Random => flags = FP_BIT_SET!(flags, fil::FP_FS_OPEN_ACCESS_RAND),
        AccessMode::Sequential => flags = FP_BIT_SET!(flags, fil::FP_FS_OPEN_ACCESS_SEQ)
    }

    if fixed {
        flags = FP_BIT_SET!(flags, fil::FP_FS_OPEN_FIXED);
    }

    if readonly {
        flags = FP_BIT_SET!(flags, fil::FP_FS_OPEN_READONLY);
    }

    let file_handle = file_system.open(filename, FileType::Data, flags)?;
    let fh = file_handle.clone();

    // construct new block.
    let mut new_block_handle = Arc::new(BlockHandle {
        name: filename.to_string(),
        object_id,

        file_handle,
        size: fh.size()?,
        extend_size: 0,
        extend_len: default_cfg.extend_len,

        sync_on_checkpoint: true,
        remote: false,
        readonly,

        allocation_size: if allocation_size == 0 {
            default_cfg.allocation_size
        } else {
            allocation_size
        },
        alloc_first: AtomicBool::new(default_cfg.alloc_first),

        // os_cache_max: default_cfg.os_cache_max,
        // os_cache_dirty_max: default_cfg.os_cache_dirty_max,

    });
    file_header_read_and_verify(new_block_handle.clone(), allocation_size)?;

    Ok(new_block_handle)

}

/**
 * close block handle
 */
fn close(file_system: Arc<FPFileSystem>, block_handle: Arc<BlockHandle>) -> FPResult<()> {
    file_system.close_fh(block_handle.name.as_str())
}

/**
 * Read and verify Meta block.
 */
fn file_header_read_and_verify(block_handle: Arc<BlockHandle>, allocation_size: FPFileSize) -> FPResult<()> {

    if block_handle.size < allocation_size as u64 {
        return Err(FP_BK_DATA_CORRUPTION)
    }

    //TODO: Metrix for the read func.

    let (mut buf, len) = block_handle.file_handle.read_exact(0, allocation_size)?;

    // Create new FileHeader.
    let mut file_header = FP_REINTERPRET_CAST_BUF_MUT!(buf, FileHeader);

    // file_header.maybe_convert_endian();
    
    let save_checksum = file_header.checksum;
    let real_checksum = if cfg!(target_endian = "big") { BIT_REVERSE_32!(file_header.checksum) } else { file_header.checksum };

    //TODO: checksum verify.
    file_header.checksum = 0;

    file_header.checksum = save_checksum;

    file_header.maybe_convert_endian();

    if file_header.magic != FP_BLOCK_MAGIC {
        return Err(FP_BK_INVALID_MAGIC)
    }

    if file_header.major > FP_BLOCK_MAJOR {
        return Err(FP_BK_INVALID_MAJOR)
    }

    if file_header.major == FP_BLOCK_MAJOR && file_header.minor > FP_BLOCK_MINOR {
        return Err(FP_BK_INVALID_MINOR)
    }

    //see: block_open.c line: 400 func: __desc_read
    Ok(())
}

pub(crate) fn file_header_write(file_handle: Arc<FPFileHandle>, alloc_size: u32) -> FPResult<()> {
    let mut buf = VEC_U8!(alloc_size);
    let header = FP_REINTERPRET_CAST_BUF_MUT!(buf, FileHeader);

    header.magic = FP_BLOCK_MAGIC;
    header.major = FP_BLOCK_MAJOR;
    header.minor = FP_BLOCK_MINOR;
    header.magic = FP_BLOCK_MAGIC;

    header.maybe_convert_endian();

    //TODO: calculate checksum.

    file_handle.write(0, alloc_size as u64, &buf)
}