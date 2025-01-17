#![allow(unused)]

use std::{sync::{Arc, Weak}, task::Context};

use crate::block::manager::BlockManager;

enum BTreePageOriented {
    ColumnFix,
    ColumnVar,
    Row
}

enum BTreePageType {
    Internal,
    Leaf,
    Root,
}

enum BtreePageStatus {
    Disk,
    Mem,
    Evicted,
    Locked,
}

/**
 * Internal page.
 */
struct BtreePageIntl {
    parent: Weak<BTreePageRef>,
    split_generation: u64,
    children: BtreePageChildren,
}

/**
 * Row store leaf page.
 */
struct BtreePageRow {
    key: *mut (),
}

/**
 * Fixed-length column-store leaf page.
 */
struct BtreePageColFix {
    fix_bitf: u8,
}

/**
 * Variable-length column-store leaf page.
 */
struct BtreePageColVar {

}


struct BtreePage {
    intl_page: BtreePageIntl,
    row_leaf_page: BtreePageRow,
    col_fix_leaf_page: BtreePageColFix,
    col_var_leaf_page: BtreePageColVar,


    leaf_entries: u32,
}


/**
 * A single in-memory page and state information.
 */
struct BTreePageRef {
    page: *mut BtreePage,
    flags: u8,
    
    page_status: BtreePageStatus, /* prefetch/reading */
}

/**
 * The page index held by each internal page.
 */
struct BtreePageChildren {
    entires: u32,
    deleted_entries: u32,
    indexes: Vec<BTreePageRef>,
}

struct BTree {
    r#type: BTreePageType,
    page_oriented: BTreePageOriented,

    k_format: String,
    v_format: String,
    fixed_length_field_size: u8,

    logging_file_id: u32,

    allocation_size: u32,
    max_internal_size: u32,
    max_leaf_page: u32,
    max_leaf_key: u32,
    max_leaf_value: u32,
    max_mem_page: u32,
    mem_page_split_throttle: u64,

    dictionary: u32,
    internal_key_truncate: bool,
    prefix_compression: bool,

    split_percentage: i32,

    block_header: u32,

    // block_manager: Arc<BlockManager>
}

/**
 * __wt_btree_open -> (__wt_blkcache_open,__wti_btree_tree_open)
 * creation = ckpt.raw.size == 0;
 * __btree_tree_open_empty: create a new btree.
 */
fn btree_open(ctx: &Context) {

}

struct TreeCreateOpt {

}

/*
 * Create an empty in-memory B-tree.
 */
fn btree_open_tree_create(btree: &mut BTree) {

}

fn btree_open_tree_open(ctx: &mut Context) {

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btree_open_tree_create() {
        // let result = add_numbers(2, 2);
        // assert_eq!(result, 4);
    }
}
