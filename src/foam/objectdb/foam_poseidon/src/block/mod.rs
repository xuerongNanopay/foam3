#![allow(unused)]

use std::{collections::LinkedList, sync::{Arc, RwLock}};

use crate::os::fil::FileHandle;

mod open;

static FP_BLOCK_INVALID_OFFSET: u64 = 0;


struct BlockManager {
    blocks: RwLock<LinkedList<Arc<Block>>>,
}

/**
 * Block; reference a singal file.
 * Physical representation of page.
 */
#[derive(Default)]
struct Block {
    name: String,   /* Name */
    object_id: u32,
    ref_count: std::sync::atomic::AtomicU32,

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

