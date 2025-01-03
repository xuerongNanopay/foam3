#![allow(unused)]

use super::*;
use crate::util::hash_city;
use std::collections::LinkedList;


struct DBCtx {
    blocks: LinkedList<Block>,
    block_lock: std::sync::Mutex<u32>,
}

/**
 * 1. find block from ctx.
 * 2. if no found, create a new one.
 */
fn block_open<'b>(ctx: &'b mut DBCtx, filename: &str, object_id: u32) -> Result<&'b Block, BlockErr> {

    let hash = hash_city::city_hash_64(filename, filename.len());
    let bucket = hash % 10; //TODO: bucket size should be a config

    let _guard = ctx.block_lock.lock().unwrap();
    
    for block in ctx.blocks.iter_mut() {
        if block.name == filename && block.object_id == object_id {
            // block already exits
            block.reference += 1;
            return Ok(block);
        }
    }

    // construct new block.
    let mut block: Block = Default::default();
    

    ctx.blocks.append(block);
    Ok(ctx.blocks.back_mut().unwrap())
}