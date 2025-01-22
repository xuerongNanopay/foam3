#![allow(unused)]

use std::{mem::ManuallyDrop, ptr, sync::atomic::AtomicUsize};

use crate::util::ptr::layout_ptr::LayoutPtr;

use super::row::RowKeyMem;

#[repr(C)]
enum BTreePageType {
    ColumnFix,
    ColumnVar,
    ColumnIntl,
    RowIntl,
    RowLeaf,
    Overflow,
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



#[repr(C)]
union BtreePageContent {
    row_intl: ManuallyDrop<BtreePageIntl>,
    /* no need to clean it */
    row_leaf: *mut BtreePageRow,
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
 * Row store leaf page.
 */
#[repr(C)]
struct BtreePageRow {
    key: *mut (), /* key in the row store leaf page. */
}


#[repr(C)]
struct BtreePage {
    r#type: BTreePageType,
    // read_gen: EvictRule,
    entries: usize, /* Leaf page entries */
    content: BtreePageContent,
    // row_leaf_page: BtreePageRow,
    // col_fix_leaf_page: BtreePageColFix,
    // col_var_leaf_page: BtreePageColVar,


    // leaf_entries: u32,
}
