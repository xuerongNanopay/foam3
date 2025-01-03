#![allow(unused)]

use super::*;
use crate::os::fs::{self, AccessMode};
use crate::util::hash_city;
use std::collections::LinkedList;
use std::sync::{Mutex, Arc};
use std::sync::atomic::{AtomicUsize, Ordering};


struct DBCtx {
    blocks: Mutex<LinkedList<Arc<Block>>>,
    // file_system: 
}

struct BlockOpenCfg {
    allocation_size: u32,
    alloc_first: bool,
    os_cache_max: usize,
    os_cache_dirty_max: usize,
    extend_len: usize,
    access_mode: fs::AccessMode,
}



/**
 * 1. find block from ctx.
 * 2. if no found, create a new one.
 */
fn block_open<>(
    ctx: &mut DBCtx, 
    default_cfg: BlockOpenCfg, 
    filename: &str, 
    object_id: u32,
    allocation_size: u32,
    readonly: bool,
    fixed: bool,
) -> Result<Arc<Block>, BlockErr> {

    let hash = hash_city::city_hash_64(filename, filename.len());
    let bucket = hash % 10; //TODO: bucket size should be a config

    let mut blocks: std::sync::MutexGuard<'_, LinkedList<Arc<Block>>> = ctx.blocks.lock().unwrap();
    
    for block in blocks.iter() {
        if block.name == filename && block.object_id == object_id {
            // block already exits
            block.ref_count.fetch_add(1, Ordering::SeqCst);
            return Ok(Arc::clone(block));
        }
    }

    // construct new block.
    let mut new_block = Block {
        name: filename.to_string(),
        object_id,
        ref_count: std::sync::atomic::AtomicU32::new(1),
        allocation_size: if allocation_size == 0 {
            default_cfg.allocation_size
        } else {
            allocation_size
        },
        alloc_first: default_cfg.alloc_first,
        os_cache_max: default_cfg.os_cache_max,
        os_cache_dirty_max: default_cfg.os_cache_dirty_max,
        extend_len: default_cfg.extend_len,
        ..Default::default()
    };

    let mut flag = 0u32;

    match default_cfg.access_mode {
        AccessMode::Random => flag = BIT_SET!(flag, fs::FP_FS_OPEN_ACCESS_RAND),
        AccessMode::Sequential => flag = BIT_SET!(flag, fs::FP_FS_OPEN_ACCESS_SEQ)
    }

    if fixed {
        flag = BIT_SET!(flag, fs::FP_FS_OPEN_FIXED);
    }

    if readonly {
        flag = BIT_SET!(flag, fs::FP_FS_OPEN_READONLY);
        new_block.readonly = true;
    }

    //TODO: open file.

    blocks.push_back(Arc::new(new_block));
    Ok(Arc::clone(blocks.back().unwrap()))
}