#![allow(unused)]

use std::{ptr, sync::{Arc, Weak}, task::Context};

use crate::{block::manager::BlockManager, types::FPResult, util::ptr::layout_ptr::LayoutPtr, FP_SIZE_OF};

#[derive(Copy, Clone, Debug)]
enum BTreeStoreOriented {
    ColumnFix,
    ColumnVar,
    Row
}

impl std::fmt::Display for BTreeStoreOriented {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BTreeStoreOriented::ColumnFix => write!(f, "ColumnFix"),
            BTreeStoreOriented::ColumnVar => write!(f, "ColumnVar"),
            BTreeStoreOriented::Row => write!(f, "Row"),
        }
    }
}

enum BTreePageType {
    ColumnFix,
    ColumnVar,
    ColumnIntl,
    RowIntl,
    RowLeaf,
    Overflow,
}

enum BtreePageStatus {
    Disk,
    Mem,
    Evicted,
    Locked,
}

enum EvictRule {
    NotSet,
    EvictSoon,
    WontNeed,
}

/**
 * Row store Internal page.
 */
struct BtreePageIntl {
    parent: * const BTreePageRef,
    // split_generation: u64,
    // children: BtreePageChildren,
}

/**
 * Row store leaf page.
 */
struct BtreePageRow {
    key: *mut (), /* key in the row store leaf page. */
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

enum BtreePageContent {
    RowIntl(BtreePageIntl),
    RowLeaf(BtreePageRow),
}

// #[derive(Default)]
struct BtreePage {
    r#type: BTreePageType,
    read_gen: EvictRule,
    entries: u32, /* Leaf page entries */

    content: BtreePageContent,
    // row_leaf_page: BtreePageRow,
    // col_fix_leaf_page: BtreePageColFix,
    // col_var_leaf_page: BtreePageColVar,


    // leaf_entries: u32,
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
    store_oriented: BTreeStoreOriented,

    // k_format: String,
    // v_format: String,
    // fixed_length_field_size: u8,

    // logging_file_id: u32,

    // allocation_size: u32,
    // max_internal_size: u32,
    // max_leaf_page: u32,
    // max_leaf_key: u32,
    // max_leaf_value: u32,
    // max_mem_page: u32,
    // mem_page_split_throttle: u64,

    // dictionary: u32,
    // internal_key_truncate: bool,
    // prefix_compression: bool,

    // split_percentage: i32,

    // block_header: u32,

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
    let root: BtreePage;
    let page_ref: BTreePageRef;

    match btree.store_oriented {
        BTreeStoreOriented::ColumnFix | BTreeStoreOriented::ColumnVar => {

        },
        BTreeStoreOriented::Row => {

        },
    };
}

fn btree_open_tree_open(ctx: &mut Context) {

}

/**
 * Create or read a page.
 */
fn btree_page_alloc(
    page_type: BTreePageType,
    alloc_entries: u32,
) -> FPResult<LayoutPtr<BtreePage>> {

    let mut size = FP_SIZE_OF!(BtreePage);

    let entries;
    let content;

    match page_type {
        BTreePageType::RowIntl | BTreePageType::ColumnFix | BTreePageType::ColumnIntl => {},
        BTreePageType::RowLeaf => size += alloc_entries as usize * FP_SIZE_OF!(BtreePageRow),
        _ => panic!("not support"),
    }

    match page_type {
        BTreePageType::RowLeaf => {
            entries = alloc_entries;
            content = BtreePageContent::RowLeaf(BtreePageRow{
                key: ptr::null_mut(),
            });
        },
        BTreePageType::RowIntl => {
            entries = alloc_entries;
            content = BtreePageContent::RowIntl(BtreePageIntl{
                parent: ptr::null_mut(),
            });
        },
        _ => panic!("unsupport"),
    };

    let page = BtreePage {
        r#type: page_type,
        read_gen: EvictRule::NotSet,
        entries,
        content,
    };

    Ok(page)
}

#[cfg(test)]
mod tests {
    use crate::{FP_SIZE_OF};

    use super::*;

    #[test]
    fn test_btree_open_tree_create() {
        let mut btree = BTree{
            store_oriented: BTreeStoreOriented::Row,
        };
        btree_open_tree_create(&mut btree)
    }

    #[test]
    fn test_btree_page_alloc() {
        enum Foo {
            A(i32),
            B(i64),
        }
        print!("size of Foo: {} bytes", FP_SIZE_OF!(Foo))
    }

}
