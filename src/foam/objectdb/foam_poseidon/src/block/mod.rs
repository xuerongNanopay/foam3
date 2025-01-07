#![allow(unused)]

use std::{collections::LinkedList, sync::{Arc, RwLock}};

use crate::{os::fil::FileHandle, types::FPConcurrentHashMap};

mod open;

static FP_BLOCK_INVALID_OFFSET: u64 = 0;


struct BlockManager {
    blocks: FPConcurrentHashMap<String, Arc<Block>>
}

impl BlockManager {
    fn get_block(&self, filename: &str) -> Option<Arc<Block>> {
        let blocks = self.blocks.read().unwrap();
        match blocks.get(filename) {
            Some(v) => Some(Arc::clone(v)),
            None => None
        }
    }
    fn insert_block(&self, filename: &str, block: Arc<Block>) {
        let mut blocks = self.blocks.write().unwrap();
        blocks.insert(String::from(filename), block);
    }
}

/**
 * Block; reference a singal file.
 * Physical representation of page.
 */
#[derive(Default)]
struct Block {
    name: String,   /* Name */
    object_id: u32,

    // filehandle: Arc<dyn FileHandle>,
    size: u64,       /* File size */
    os_cache: usize,

    allocation_size: u32,
    alloc_first: bool,
    os_cache_max: usize,
    os_cache_dirty_max: usize,
    extend_len: usize,
    
    readonly: bool,
}

enum BlockErr {
    NoFound
}

