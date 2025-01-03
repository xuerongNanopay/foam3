#![allow(unused)]

use super::*;
use crate::util::hash_city;
use std::collections::LinkedList;
use std::sync::{Mutex, Arc};


struct DBCtx {
    blocks: LinkedList<Arc<Block>>,
    block_lock: std::sync::Mutex<u32>,
}

struct BlockOpenCfg {
    allocation_size: u32,
    alloc_first: bool,
    os_cache_max: usize,
    os_cache_dirty_max: usize,
    extend_len: usize,
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
) -> Result<Arc<Block>, BlockErr> {

    let hash = hash_city::city_hash_64(filename, filename.len());
    let bucket = hash % 10; //TODO: bucket size should be a config

    let _guard = ctx.block_lock.lock().unwrap();
    
    for block in ctx.blocks.iter() {
        if block.name == filename && block.object_id == object_id {
            // block already exits
            *block.reference.lock().unwrap() += 1;
            return Ok(Arc::clone(block));
        }
    }

    // construct new block.
    let mut new_block = Block {
        name: filename.to_string(),
        object_id,
        reference: Mutex::new(1),
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

    //TODO: open file handler;

    ctx.blocks.push_back(Arc::new(new_block));
    Ok(Arc::clone(ctx.blocks.back().unwrap()))
}