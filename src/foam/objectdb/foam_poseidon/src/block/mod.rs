#![allow(unused)]

mod open;

static FP_BLOCK_INVALID_OFFSET: u64 = 0;

/**
 * Block; reference a singal file.
 * Physical representation of page.
 */
#[derive(Default)]
struct Block {
    name: String,   /* Name */
    object_id: u32,
    reference: u32,

    size: u64,       /* File size */
    os_cache: usize,
}

enum BlockErr {
    NoFound
}