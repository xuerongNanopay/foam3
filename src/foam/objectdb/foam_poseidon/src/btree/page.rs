#![allow(unused)]

use std::{mem::ManuallyDrop, ptr, sync::atomic::AtomicUsize};

use crate::{types::FPResult, util::ptr::layout_ptr::LayoutPtr, FP_ALLOC, FP_SIZE_OF};

use super::{btree::BTreeKey, row::{RowIntl, RowKeyMem, RowLeaf}};

/**
 * PageRef type.
 */

 #[repr(usize)]
pub(super) enum PageRefType {
    Internal = 0,
    Leaf = 1,
}

 #[repr(usize)]
 pub(super) enum PageRefState {
    Disk = 0,    /* Page is on disk. */
    Deleted = 1, /* Page is on disk, but deleted */
    Locked = 2,  /* Page locked for exclusive access */
    Mem = 3,     /* Page is in cache and memory */
    Split = 4,   /* Parent page split */
}

/**
 * A wrapper for page, store/keep the metadate for b-tree page.
 */
#[repr(C)]
pub(super) struct PageRef {
    pub(super) page: Option<LayoutPtr<Page>>,
    pub(super) home: *const Page,
    pub(super) addr: *const (),
    pub(super) unused: u8,
    pub(super) r#type: PageRefType,
    pub(super) state: AtomicUsize,
    
    pub(super) key: BTreeKey,
    // page_status: pageStatus, /* prefetch/reading */
}



#[repr(C)]
pub(super) union PageContent {
    pub(super) row_intl: ManuallyDrop<RowIntl>,
    /* no need to clean it */
    pub(super) row_leaf: *mut RowLeaf,
}

/**
 * The page index held by each internal page.
 */
#[repr(C)]
pub(super) struct PageIndex {
    pub(super) entries: usize,
    pub(super) deleted_entries: usize,
    pub(super) page_refs: *mut LayoutPtr<PageRef>,
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
pub(super) enum PageType {
    ColumnFix,
    ColumnVar,
    ColumnIntl,
    RowIntl,
    RowLeaf,
    Overflow,
}

#[repr(C)]
pub(super) struct Page {
    pub(super) r#type: PageType,
    // read_gen: EvictRule,
    pub(super) entries: usize, /* Leaf page entries */
    pub(super) content: PageContent,
    // row_leaf_page: BtreePageRow,
    // col_fix_leaf_page: BtreePageColFix,
    // col_var_leaf_page: BtreePageColVar,


    // leaf_entries: u32,
}

impl Drop for Page {
    fn drop(&mut self) {
        if let PageType::RowIntl = self.r#type {
            unsafe {
                ManuallyDrop::drop(&mut self.content.row_intl);
            }
        }
        if let PageType::RowLeaf = self.r#type {
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

impl Page {

    /**
     * Create or read a page.
     */
    pub(crate) fn new(
        page_type: PageType,
        alloc_entries: usize,
        is_alloc_page_refs: bool,
    ) -> FPResult<(LayoutPtr<Page>)> {

        let mut mem_size: usize = 0;
    
        let tree_page: LayoutPtr<Page> = match page_type {
            // Create tree page for row internal page.
            PageType::RowIntl => {
                // Create tree page.
                let (l1, p_tree_page) = FP_ALLOC!{
                    Page: 1,
                }?;
                let mut tree_page = LayoutPtr::new(l1, p_tree_page);
                mem_size += FP_SIZE_OF!(Page);

                // Create index for row store internal page.
                let (l2, p_page_index, page_refs) = FP_ALLOC!{
                    PageIndex: 1,
                    LayoutPtr<PageRef>: alloc_entries,
                }?;
                mem_size += FP_SIZE_OF!(PageIndex) + alloc_entries * FP_SIZE_OF!(* mut PageRef);
                
                let mut page_index = LayoutPtr::new(l2, p_page_index);

                unsafe {
                    page_index.entries = alloc_entries;
                    page_index.page_refs = page_refs;
                    tree_page.content = PageContent {
                        row_intl:  ManuallyDrop::new(RowIntl{
                            parent: ptr::null_mut(),
                            split_generation: 0,
                            page_index,
                        })
                    };

                    if is_alloc_page_refs {
                        for i in 0..alloc_entries {
                            let c_ptr: *mut LayoutPtr<PageRef> = (*p_page_index).page_refs.add(i);
                            let (l, p) = FP_ALLOC!{
                                PageRef: 1,
                            }?;
                            *c_ptr = LayoutPtr::new(l, p);
                            mem_size += FP_SIZE_OF!(PageRef);
                        }
                    }
                }

                tree_page
            },
            // Create tree page for row leaf page.
            PageType::RowLeaf => {
                let (l1, p_tree_page, p_page_row) = FP_ALLOC!{
                    Page: 1,
                    RowLeaf: alloc_entries,
                }?;
                let mut tree_page = LayoutPtr::new(l1, p_tree_page);
                mem_size += FP_SIZE_OF!(Page);

                unsafe {
                    (*tree_page).content = PageContent {
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