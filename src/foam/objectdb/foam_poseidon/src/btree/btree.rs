#![allow(unused)]

use std::{mem::ManuallyDrop, ptr, str::FromStr, sync::{atomic::{AtomicUsize, Ordering}, Arc, Weak}, task::Context};

use crate::{block::manager::BlockManager, error::FP_NO_SUPPORT, types::FPResult, util::ptr::layout_ptr::LayoutPtr, FP_ALLOC, FP_BIT_IS_SET, FP_SIZE_OF};

use super::row::RowKeyMem;

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
struct BtreePageIntl {
    parent: *const BTreePageRef,
    split_generation: u64,
    page_index: LayoutPtr<BTreePageIndex>,
}

/**
 * Row store leaf page.
 */
#[repr(C)]
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
    row_intl: ManuallyDrop<BtreePageIntl>,
    /* no need to clean it */
    row_leaf: *mut BtreePageRow,
}


/**
 * The page index held by each internal page.
 */
#[repr(C)]
struct BTreePageIndex {
    entries: usize,
    deleted_entries: usize,
    page_refs: *mut LayoutPtr<BTreePageRef>,
}

impl Drop for BTreePageIndex {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.entries {
                let mut p = self.page_refs.add(i);
                if !p.is_null() {
                    ptr::drop_in_place(p);
                    p = ptr::null_mut();
                }

            }
        }
    }
}

/**
 * BTreePageRef type.
 */

 #[repr(usize)]
enum BTreePageRefType {
    Internal = 0,
    Leaf = 1,
}

/**
 * BTreePageRef State.
 */

 #[repr(usize)]
enum BTreePageRefState {
    Disk = 0,    /* Page is on disk. */
    Deleted = 1, /* Page is on disk, but deleted */
    Locked = 2,  /* Page locked for exclusive access */
    Mem = 3,     /* Page is in cache and memory */
    Split = 4,   /* Parent page split */
}

/**
 * Representation for b-tree key.
 */
#[repr(C)]
enum BTreeKey {
    Row(*mut ()), /* row store */
    RowMem(RowKeyMem), /* In-memory row key */
    Col(u64),     /* column */
}

/**
 * A wrapper for BtreePage, store/keep the metadate for b-tree page.
 */
#[repr(C)]
struct BTreePageRef {
    page: Option<LayoutPtr<BtreePage>>,
    home: *const BtreePage,
    addr: *const (),
    unused: u8,
    r#type: BTreePageRefType,
    state: AtomicUsize,
    
    key: BTreeKey,
    // page_status: BtreePageStatus, /* prefetch/reading */
}

pub const BTREE_BULK: u32 = 1 << 12;

#[repr(C)]
struct BTree {
    store_oriented: BTreeStoreOriented,

    /* Store oriented: row, var length column, fix length column */
    r#type: BTreeType,

    /* Root page reference. */
    root: BTreePageRef,

    flag: u32,
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

#[repr(C)]
enum BTreeType {
    ColumnFix,
    ColumnVar,
    Row,
}

impl BTree {

    /*
    * Initial an empty in-memory B-tree.
    */
    fn initial_empty(btree: &mut LayoutPtr<BTree>) -> FPResult<()> {
        match btree.r#type {
            BTreeType::Row => {
                // First b-tree page(Internal).
                let mut root_page: LayoutPtr<BtreePage> = BtreePage::new(BTreePageType::RowIntl, 1, true)?;
                unsafe {
                    // root page parent is root page ref.
                    (*root_page.content.row_intl).parent = &btree.root as *const BTreePageRef;

                    // Initial leaf page ref.
                    let first_page_ref: *mut LayoutPtr<BTreePageRef> = root_page.content.row_intl.page_index.page_refs;
                    (*first_page_ref).home = &*root_page;
                    (*first_page_ref).page = None;
                    (*first_page_ref).addr = ptr::null();
                    (*first_page_ref).r#type = BTreePageRefType::Leaf;
                    (*first_page_ref).state.store(BTreePageRefState::Deleted as usize, Ordering::SeqCst);
                    // Give the a initial key `""`
                    (*first_page_ref).key = BTreeKey::RowMem(Self::init_mem_row_key("")?);


                    // Initial first leaf page if bulk load on.
                    if FP_BIT_IS_SET!(btree.flag, BTREE_BULK) {
                        Self::new_leaf_page(btree, &mut *first_page_ref)?;
                        (*first_page_ref).r#type = BTreePageRefType::Leaf;
                        (*first_page_ref).state.store(BTreePageRefState::Mem as usize, Ordering::SeqCst);
                        //TODO: BtreePageModify.
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
    fn root_ref_init(btree: &mut LayoutPtr<BTree>, mut root_page: LayoutPtr<BtreePage>, is_col_store: bool) {

        unsafe {
            /* root internal page parent point to root ref in btree, another word root parent is root. */
            (*root_page.content.row_intl).parent = &btree.root as *const BTreePageRef;   
        }

        btree.root.page = Some(root_page);
        btree.root.r#type = BTreePageRefType::Internal;
        btree.root.state.store(BTreePageRefState::Mem as usize, Ordering::SeqCst);


        if is_col_store {
            btree.root.key = BTreeKey::Col(1);
        }
    }

    /**
     * Create a leaf page for both row and column store.
     */
    fn new_leaf_page(btree: & LayoutPtr<BTree>, page_ref: &mut LayoutPtr<BTreePageRef>) -> FPResult<()> {

        let page = match btree.r#type {
            BTreeType::Row => {
                BtreePage::new(BTreePageType::RowLeaf, 0, false)
            },
            _ => {
                return Err(FP_NO_SUPPORT)
            }
        }?;
        page_ref.page = Some(page);
        page_ref.r#type = BTreePageRefType::Leaf;

        Ok(())
    }

    fn init_mem_row_key(key: &str) -> FPResult<RowKeyMem>{
        //TODO: memory metric
        RowKeyMem::from_str(key)
    }

    /*
    * Open an existed B-tree.
    */
    fn open(ctx: &mut Context) {

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

impl Drop for BtreePage {
    fn drop(&mut self) {
        if let BTreePageType::RowIntl = self.r#type {
            unsafe {
                ManuallyDrop::drop(&mut self.content.row_intl);
            }
        }
        if let BTreePageType::RowLeaf = self.r#type {
            unsafe {
                for i in 0..self.entries {
                    let mut p = self.content.row_leaf.add(i);
                    if !p.is_null() {
                        ptr::drop_in_place(p);
                        p = ptr::null_mut();
                    }
    
                }
            }
        }
    }
}

impl BtreePage {

    /**
     * Create or read a page.
     */
    pub(crate) fn new(
        page_type: BTreePageType,
        alloc_entries: usize,
        is_alloc_page_refs: bool,
    ) -> FPResult<(LayoutPtr<BtreePage>)> {

        let mut mem_size: usize = 0;
    
        let tree_page: LayoutPtr<BtreePage> = match page_type {
            // Create tree page for row internal page.
            BTreePageType::RowIntl => {
                // Create tree page.
                let (l1, p_tree_page) = FP_ALLOC!{
                    BtreePage: 1,
                }?;
                let mut tree_page = LayoutPtr::new(l1, p_tree_page);
                mem_size += FP_SIZE_OF!(BtreePage);

                // Create index for row store internal page.
                let (l2, p_page_index, page_refs) = FP_ALLOC!{
                    BTreePageIndex: 1,
                    LayoutPtr<BTreePageRef>: alloc_entries,
                }?;
                mem_size += FP_SIZE_OF!(BTreePageIndex) + alloc_entries * FP_SIZE_OF!(* mut BTreePageRef);
                
                let mut page_index = LayoutPtr::new(l2, p_page_index);

                unsafe {
                    page_index.entries = alloc_entries;
                    page_index.page_refs = page_refs;
                    tree_page.content = BtreePageContent {
                        row_intl:  ManuallyDrop::new(BtreePageIntl{
                            parent: ptr::null_mut(),
                            split_generation: 0,
                            page_index,
                        })
                    };

                    if is_alloc_page_refs {
                        for i in 0..alloc_entries {
                            let c_ptr: *mut LayoutPtr<BTreePageRef> = (*p_page_index).page_refs.add(i);
                            let (l, p) = FP_ALLOC!{
                                BTreePageRef: 1,
                            }?;
                            *c_ptr = LayoutPtr::new(l, p);
                            mem_size += FP_SIZE_OF!(BTreePageRef);
                        }
                    }
                }

                tree_page
            },
            // Create tree page for row leaf page.
            BTreePageType::RowLeaf => {
                let (l1, p_tree_page, p_page_row) = FP_ALLOC!{
                    BtreePage: 1,
                    BtreePageRow: alloc_entries,
                }?;
                let mut tree_page = LayoutPtr::new(l1, p_tree_page);
                mem_size += FP_SIZE_OF!(BtreePage);

                unsafe {
                    (*tree_page).content = BtreePageContent {
                        row_leaf: p_page_row,
                    };
                    (*tree_page).entries= alloc_entries;
                }

                tree_page
            }
            _ => panic!("not support"),
        };

        Ok(tree_page)
    }

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