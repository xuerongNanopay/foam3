#![allow(unused)]

use std::{mem::ManuallyDrop, ptr, sync::atomic::AtomicUsize};

use crate::util::ptr::layout_ptr::LayoutPtr;

use super::{btree::BTreeKey, row::{RowIntl, RowKeyMem, RowLeaf}};

/**
 * PageRef type.
 */

 #[repr(usize)]
enum PageRefType {
    Internal = 0,
    Leaf = 1,
}

/**
 * A wrapper for page, store/keep the metadate for b-tree page.
 */
#[repr(C)]
pub(super) struct PageRef {
    page: Option<LayoutPtr<Page>>,
    home: *const Page,
    addr: *const (),
    unused: u8,
    r#type: PageRefType,
    state: AtomicUsize,
    
    key: BTreeKey,
    // page_status: pageStatus, /* prefetch/reading */
}



#[repr(C)]
pub(super) union PageContent {
    row_intl: ManuallyDrop<RowIntl>,
    /* no need to clean it */
    row_leaf: *mut RowLeaf,
}

/**
 * The page index held by each internal page.
 */
#[repr(C)]
pub(super) struct PageIndex {
    entries: usize,
    deleted_entries: usize,
    page_refs: *mut LayoutPtr<PageRef>,
}

impl Drop for PageIndex {
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


#[repr(C)]
enum PageType {
    ColumnFix,
    ColumnVar,
    ColumnIntl,
    RowIntl,
    RowLeaf,
    Overflow,
}

#[repr(C)]
struct Page {
    r#type: PageType,
    // read_gen: EvictRule,
    entries: usize, /* Leaf page entries */
    content: PageContent,
    // row_leaf_page: BtreePageRow,
    // col_fix_leaf_page: BtreePageColFix,
    // col_var_leaf_page: BtreePageColVar,


    // leaf_entries: u32,
}
