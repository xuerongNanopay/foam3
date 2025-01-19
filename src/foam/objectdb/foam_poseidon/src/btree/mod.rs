#![allow(unused)]

use std::{ptr, sync::{Arc, Weak}, task::Context};

use crate::{block::manager::BlockManager, types::FPResult, util::ptr::layout_ptr::LayoutPtr, FP_ALLOC, FP_SIZE_OF};

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

#[repr(C)]
enum BTreePageType {
    ColumnFix,
    ColumnVar,
    ColumnIntl,
    RowIntl,
    RowLeaf,
    Overflow,
}

#[repr(C)]
enum BtreePageStatus {
    Disk,
    Mem,
    Evicted,
    Locked,
}

#[repr(C)]
enum EvictRule {
    NotSet,
    EvictSoon,
    WontNeed,
}

/**
 * Row store Internal page.
 */
#[repr(C)]
#[derive(Copy, Clone)]
struct BtreePageIntl {
    parent: *mut BTreePageRef,
    split_generation: u64,
    page_index: *mut BTreePageIndex,
}

/**
 * Row store leaf page.
 */
#[repr(C)]
#[derive(Copy, Clone)]
struct BtreePageRow {
    key: *mut (), /* key in the row store leaf page. */
}

/**
 * Fixed-length column-store leaf page.
 */
#[repr(C)]
 struct BtreePageColFix {
    fix_bitf: u8,
}

/**
 * Variable-length column-store leaf page.
 */
#[repr(C)]
 struct BtreePageColVar {

}

#[repr(C)]
union BtreePageContent {
    row_intl: BtreePageIntl,
    row_leaf: BtreePageRow,
}

// #[derive(Default)]
#[repr(C)]
struct BtreePage {
    r#type: BTreePageType,
    read_gen: EvictRule,
    entries: usize, /* Leaf page entries */

    content: BtreePageContent,
    // row_leaf_page: BtreePageRow,
    // col_fix_leaf_page: BtreePageColFix,
    // col_var_leaf_page: BtreePageColVar,


    // leaf_entries: u32,
}

/**
 * The page index held by each internal page.
 */
#[repr(C)]
 struct BTreePageIndex {
    entries: usize,
    deleted_entries: usize,
    
    page_refs: *mut *mut BTreePageRef,
}

/**
 * A single in-memory page and state information.
 */
#[repr(C)]
 struct BTreePageRef {
    page: *mut BtreePage,
    unused: u8,
    flags: u8,
    
    page_status: BtreePageStatus, /* prefetch/reading */
}

#[repr(C)]
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

impl BtreePage {

    /**
     * Create or read a page.
     */
    fn btree_page_alloc(
        page_type: BTreePageType,
        alloc_entries: usize,
        is_alloc_page_refs: bool,
    ) -> FPResult<()> {

        // let mut size = FP_SIZE_OF!(BtreePage);
        let mut mem_size: usize = 0;

        let entries;
        // let content;

        // let content = BtreePageContent {
        //     p_row_intl: ptr::null_mut(),
        // };
    
        let tree_page = match page_type {
            // Create tree page for row internal page.
            BTreePageType::RowIntl => {
                let (l1, tree_page) = FP_ALLOC!{
                    BtreePage: 1,
                };

                let (l2, page_index, page_refs) = FP_ALLOC!{
                    BTreePageIndex: 1,
                    * mut BTreePageRef: alloc_entries,
                };
                mem_size += FP_SIZE_OF!(BTreePageIndex) + alloc_entries * FP_SIZE_OF!(* mut BTreePageRef);
                
                unsafe {
                    (*page_index).entries = alloc_entries;
                    (*page_index).page_refs = page_refs;
                    //TODO: #![feature(asm)] atomic store. Release Boundary. Need?
                    (*tree_page).content.row_intl.page_index = page_index;

                    if is_alloc_page_refs {
                        for i in 0..alloc_entries {
                            let c_ptr = (*page_index).page_refs.add(i);
                            
                            mem_size += FP_SIZE_OF!(BTreePageRef)
                        }
                    }
                }
            }
            _ => panic!("not support"),
        };

        // let (layout, mut btree, ) = FP_ALLOC!{
        //     BtreePage: 1,
        //     BtreePageRow: entries,
        // };

        match page_type {
            BTreePageType::RowLeaf => {
                entries = alloc_entries;
                // content = BtreePageContent::RowLeaf(BtreePageRow{
                //     key: ptr::null_mut(),
                // });
            },
            BTreePageType::RowIntl => {
                entries = alloc_entries;
                // content = BtreePageContent::RowIntl(BtreePageIntl{
                //     parent: ptr::null_mut(),
                // });
            },
            _ => panic!("unsupport"),
        };

        

        let page = BtreePage {
            r#type: page_type,
            read_gen: EvictRule::NotSet,
            entries,
            //TODO: refactor
            content: BtreePageContent {
                row_intl: BtreePageIntl{
                    parent: ptr::null_mut(),
                    split_generation: 0,
                    page_index: ptr::null_mut(),
                },
            },
        };



        Ok(())
    }

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

// use std::alloc::{alloc, dealloc, Layout};
// use std::collections::HashMap;
// use std::sync::Mutex;

// lazy_static::lazy_static! {
//     static ref MEMORY_MAP: Mutex<HashMap<*mut u8, Layout>> = Mutex::new(HashMap::new());
// }

// fn allocate(size: usize, align: usize) -> *mut u8 {
//     let layout = Layout::from_size_align(size, align).expect("Invalid layout");
//     let ptr = unsafe { alloc(layout) };
//     if ptr.is_null() {
//         panic!("Memory allocation failed");
//     }
//     MEMORY_MAP.lock().unwrap().insert(ptr, layout);
//     ptr
// }

// fn deallocate(ptr: *mut u8) {
//     let layout = MEMORY_MAP.lock().unwrap().remove(&ptr).expect("Pointer not found");
//     unsafe { dealloc(ptr, layout) };
//     println!("Memory deallocated");
// }

// fn main() {
//     let ptr = allocate(128, 16);
//     println!("Memory allocated at: {:?}", ptr);
//     deallocate(ptr);
// }