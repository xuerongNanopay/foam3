#![allow(unused)]

pub mod btree_cursor;
pub mod btree_dao;

use std::{mem::ManuallyDrop, ptr, str::FromStr, sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Weak}, task::Context};

use crate::{block::manager::BlockManager, error::FP_NO_SUPPORT, types::FPResult, util::ptr::layout_ptr::LayoutPtr, FP_ALLOC, FP_BIT_IS_SET, FP_SIZE_OF};

use super::{page::{Page, PageRef, PageRefState, PageRefType, PageType}, row::RowKeyMem};


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
pub(crate) enum BTreeType {
    ColumnFix,
    ColumnVar,
    Row,
}

#[repr(C)]
pub(crate) enum BTreeKey {
    Row(*mut ()), /* row store */
    RowMem(RowKeyMem), /* In-memory row key */
    Col(u64),     /* column */
}


pub type BtreeFlag = u32;
pub const BTREE_APPEND:    BtreeFlag = 1 << 0;
pub const BTREE_BULK:      BtreeFlag = 1 << 12;  /* Bulk-load */
pub const BTREE_CLOSED:    BtreeFlag = 1 << 13;  /* Closed */
pub const BTREE_IN_MEMORY: BtreeFlag = 1 << 14;  /* In-Memory */
pub const BTREE_RECOVER:   BtreeFlag = 1 << 15;  /* Recover */
pub const BTREE_VERIFY:    BtreeFlag = 1 << 16;  /* Verify */

#[repr(C)]
pub(crate) struct BTree {
    initial: AtomicBool,

    store_oriented: BTreeStoreOriented,

    /* Store oriented: row, var length column, fix length column */
    pub(crate) r#type: BTreeType,

    /* Root page reference. */
    root: PageRef,

    pub(crate) flags: BtreeFlag,
    
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
 * meta data.
 */
impl BTree {

    /*
    * Create or reopen a btree.
    */
    fn new(ctx: &mut Context, flag: BtreeFlag) -> FPResult<()> {
        Ok(())
    }

    /*
    * Initial an empty in-memory B-tree.
    */
    fn initial_empty(btree: &mut LayoutPtr<BTree>) -> FPResult<()> {
        match btree.r#type {
            BTreeType::Row => {
                // First b-tree page(Internal).
                let mut root_page: LayoutPtr<Page> = Page::new(PageType::RowIntl, 1, true)?;
                unsafe {
                    // root page parent is root page ref.
                    (*root_page.content.row_intl).parent = &btree.root as *const PageRef;

                    // Initial leaf page ref.
                    let first_page_ref: *mut LayoutPtr<PageRef> = root_page.content.row_intl.page_index.page_refs;
                    (*first_page_ref).home = &*root_page;
                    (*first_page_ref).page = None;
                    (*first_page_ref).addr = ptr::null();
                    (*first_page_ref).r#type = PageRefType::Leaf;
                    (*first_page_ref).state.store(PageRefState::Deleted as usize, Ordering::SeqCst);
                    // Give the a initial key `""`
                    (*first_page_ref).key = BTreeKey::RowMem(Self::row_mem_key_init("")?);


                    // Initial first leaf page if bulk load on.
                    if FP_BIT_IS_SET!(btree.flags, BTREE_BULK) {
                        Self::new_leaf_page(btree, &mut *first_page_ref)?;
                        (*first_page_ref).r#type = PageRefType::Leaf;
                        (*first_page_ref).state.store(PageRefState::Mem as usize, Ordering::SeqCst);
                        //TODO: PageModify.
                    }

                }

                Self::root_ref_init(btree, root_page, !matches!(btree.r#type,BTreeType::Row));
            },
            _ => return Err(FP_NO_SUPPORT)
        };
        Ok(())
    }

    /**
     * Initial Btree root page ref.
     */
    fn root_ref_init(btree: &mut LayoutPtr<BTree>, mut root_page: LayoutPtr<Page>, is_col_store: bool) {

        unsafe {
            /* root internal page parent point to root ref in btree, another word root parent is root. */
            (*root_page.content.row_intl).parent = &btree.root as *const PageRef;   
        }

        btree.root.page = Some(root_page);
        btree.root.r#type = PageRefType::Internal;
        btree.root.state.store(PageRefState::Mem as usize, Ordering::SeqCst);


        if is_col_store {
            btree.root.key = BTreeKey::Col(1);
        }
    }

    /**
     * Create a new leaf page for both row and column store.
     */
    pub(crate) fn new_leaf_page(btree: & LayoutPtr<BTree>, page_ref: &mut LayoutPtr<PageRef>) -> FPResult<()> {

        let page = match btree.r#type {
            BTreeType::Row => {
                Page::new(PageType::RowLeaf, 0, false)
            },
            _ => {
                return Err(FP_NO_SUPPORT)
            }
        }?;
        page_ref.page = Some(page);
        page_ref.r#type = PageRefType::Leaf;

        Ok(())
    }

    /**
     * Initial a b-tree memory key.
     */
    fn row_mem_key_init(key: &str) -> FPResult<RowKeyMem>{
        //TODO: memory metric
        RowKeyMem::from_str(key)
    }

    pub(crate) fn disable_bulk_load(&self) {
        if !*&self.initial.load(Ordering::Relaxed) {
            return;
        }
        &self.initial.store(false, Ordering::SeqCst);

        
    }

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

#[cfg(test)]
mod tests {
    use crate::{FP_SIZE_OF};

    use super::*;

    #[test]
    fn test_btree_open_tree_create() {
        // let mut btree = BTree{
        //     store_oriented: BTreeStoreOriented::Row,
        // };
        // btree_open_tree_create(&mut btree)
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