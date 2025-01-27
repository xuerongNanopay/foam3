#![allow(unused)]

use std::ptr;

use crate::{btree::{page::PageIndex, BtreeInsert, BtreeInsertList, FP_BTREE_MAX_KV_SIZE, FP_BTREE_PRIMITIVE_KEY_MAX_LEN, FP_RECORD_NUMBER_OOB}, cursor::{CursorFlag, CursorItem, ICursor, CURSOR_BOUND_LOWER, CURSOR_BOUND_LOWER_INCLUSIVE, CURSOR_BOUND_UPPER, CURSOR_BOUND_UPPER_INCLUSIVE}, dao::DAO, error::{FP_NO_IMPL, FP_NO_SUPPORT}, misc::FP_GIGABYTE, types::FPResult, util::ptr::layout_ptr::LayoutPtr, FP_BIT_CLR, FP_BIT_IS_SET};

use super::{btree_dao::BTreeDAO, BTree, BTreeType, Page, PageRef, PageType};

struct BtreeCursorState {
    key: CursorItem,
    value: CursorItem,
    record_number: u64,
    flags: CursorFlag
}

impl BtreeCursorState {
    fn from_cursor(cursor: &ICursor) -> BtreeCursorState {
        BtreeCursorState {
            record_number: cursor.record_number,
            flags: cursor.flags,
            key: cursor.key.clone(),
            value: cursor.value.clone(),
        }
    }
}

pub(crate) type BtreeCursorFlag = u32;
pub const BTREE_CURSOR_ACTIVE:               BtreeCursorFlag = 1 << 0;
pub const BTREE_CURSOR_ITER_APPEND:          BtreeCursorFlag = 1 << 2;  /* Column store: iterating append list. */
pub const BTREE_CURSOR_ITER_NEXT:            BtreeCursorFlag = 1 << 3; 
pub const BTREE_CURSOR_ITER_PREV:            BtreeCursorFlag = 1 << 4; 
pub const BTREE_CURSOR_ITER_RETRY_NEXT:      BtreeCursorFlag = 1 << 5;
pub const BTREE_CURSOR_ITER_RETRY_PREV:      BtreeCursorFlag = 1 << 6;


pub const BTREE_CURSOR_POSITION_MASK: BtreeCursorFlag = 
    BTREE_CURSOR_ITER_APPEND | 
    BTREE_CURSOR_ITER_NEXT |
    BTREE_CURSOR_ITER_PREV |
    BTREE_CURSOR_ITER_RETRY_NEXT | 
    BTREE_CURSOR_ITER_RETRY_PREV;

/**
 * Btree cursor.
 */
pub(crate) struct BtreeCursor<'a, 'b, 'c> {
    pub(crate) icur: &'a ICursor,
    pub(crate) btree: &'b mut BTree,
    btree_dao: &'c BTreeDAO<'c>,

    // Date Source.
    pub(crate) data_source: *mut(),

    /* Pointer may be better options */
    pub(crate) cur_page: *mut PageRef,
    pub(crate) slot: u32,
    
    pub(crate) record_number: u64,

    pub(crate) cur_insert_list: *mut BtreeInsertList,
    pub(crate) cur_insert: *mut BtreeInsert,

    pub(crate) flags: BtreeCursorFlag,

    pub(crate) page_ref: *mut PageRef,
}

impl BtreeCursor<'_, '_, '_> {

    /**
     * __wt_btcur_insert
     */
    pub(crate) fn insert(&mut self) -> FPResult<()> {

        let mut save_state: BtreeCursorState;

        //TODO: stats
        let insert_len = self.icur.key.len() + self.icur.value.len();

        // Verify KV length.
        if matches!(self.btree.r#type, BTreeType::Row) {
            self.size_check(&self.icur.key)?;
        }
        self.size_check(&self.icur.value)?;

        // Bulk-load only available before the first insert.
        self.btree_dao.get_btree().disable_bulk_load();

        //TODO: support append for column stored.

        //TODO: Override.

        /* Verify bounds. */
        self.is_key_within_bounds()?;

        save_state = self.save_cursor_state();

        self.init(true);

        match self.btree.r#type {
            BTreeType::Row => {
                //1. search row.
                //2. fail if duplicate if duplicate not allow.
                //3. modify row.
            },
            _ => {
                return Err(FP_NO_IMPL)
            }
        }

        Ok(())
    }

    /**
     * Check if a insert item is too large. 
     */
    fn size_check(&self, item: &CursorItem) -> FPResult<()> {

        if !matches!(self.btree.r#type, BTreeType::ColumnFix) {
            //see: __cursor_size_chk
            panic!("Do not support")
        }

        if item.len() <= FP_GIGABYTE {
            return Ok(())
        }

        if item.len() > FP_BTREE_MAX_KV_SIZE {
            return Err(FP_NO_SUPPORT)
        }

        /* Check if block hanlder can wactually write. */
        self.btree_dao.write_size(item.len())?;

        Ok(())
    }

    /**
     * Verify if a given key is in within the bounds.
     */
    fn is_key_within_bounds(&self) -> FPResult<bool> {

        let mut ret= false;

        /* Ignore if not config */
        if FP_BIT_IS_SET!(self.icur.flags, CURSOR_BOUND_LOWER | CURSOR_BOUND_UPPER){
            return Ok(true);
        }

        //TODO: unlikely assert.

        if FP_BIT_IS_SET!(self.icur.flags, CURSOR_BOUND_LOWER) && false {
            ret = self.compare_bounds(false)?;
        }

        if FP_BIT_IS_SET!(self.icur.flags, CURSOR_BOUND_UPPER) {
            ret = self.compare_bounds(true)?;
        }

        Ok(ret)
    }

    fn compare_bounds(&self, upper: bool) -> FPResult<bool> {
        let mut record_number_bound: u64;
        let mut cmp: i32;
        if upper {
            if matches!(self.btree.r#type, BTreeType::Row) {
                cmp = self.btree.key_order.compare(&self.icur.key, &self.btree.upper_bound);
            } else {
                //TODO: column.
                return Err(FP_NO_IMPL);
            }

            if FP_BIT_IS_SET!(self.icur.flags, CURSOR_BOUND_UPPER_INCLUSIVE) {
                if matches!(self.btree.r#type, BTreeType::Row) {
                    return Ok(cmp > 0);
                } else {
                    return Err(FP_NO_IMPL);
                }
            } else {
                if matches!(self.btree.r#type, BTreeType::Row) {
                    return Ok(cmp >= 0);
                } else {
                    return Err(FP_NO_IMPL);
                }
            }
        } else {
            if matches!(self.btree.r#type, BTreeType::Row) {
                cmp = self.btree.key_order.compare(&self.icur.key, &self.btree.lower_bound);

            } else {
                return Err(FP_NO_IMPL);
            }

            if FP_BIT_IS_SET!(self.icur.flags, CURSOR_BOUND_LOWER_INCLUSIVE) {
                if matches!(self.btree.r#type, BTreeType::Row) {
                    return Ok(cmp < 0);
                } else {
                    return Err(FP_NO_IMPL);
                }
            } else {
                if matches!(self.btree.r#type, BTreeType::Row) {
                    return Ok(cmp <= 0);
                } else {
                    return Err(FP_NO_IMPL);
                }
            }
        }
        Ok(false)
    }

    /**
     * Initial cursor.
     */
    fn init(&mut self, reenter: bool) -> FPResult<()> {

        if reenter {
            self.reset()?;
        }


        Ok(())
        
    }

    /**
     * Reset cursor.
     * __cursor_reset
     */
    fn reset(&mut self) -> FPResult<()> {
        self.clear_position()?;

        /* Deactive the cursor. */
        if FP_BIT_IS_SET!(self.flags, BTREE_CURSOR_ACTIVE) {
            FP_BIT_CLR!(self.flags, BTREE_CURSOR_ACTIVE)
        }

        if self.page_ref.is_null() {
            return Ok(())
        }

        //TODO: more
        Ok(())
    }

    /**
     * Clear position.
     */
    fn clear_position(&mut self) -> FPResult<()> {
        self.record_number = FP_RECORD_NUMBER_OOB;

        self.cur_insert = ptr::null_mut();
        self.cur_insert_list = ptr::null_mut();

        FP_BIT_CLR!(self.flags, BTREE_CURSOR_POSITION_MASK);

        Ok(())
    }

    /**
     * Row-store search from a cursor.
     * __cursor_row_search, __wt_row_search
     */
    fn row_search(&mut self, insert: bool, leaf_safe: bool) -> FPResult<Option<*mut PageRef>> {
        //TODO: page lock/resource generation manage.

        // let mut current: *mut PageRef = ptr::null_mut();

        self.clear_position();

        //TODO: support column append.

        /* Search b-tree from the root */
        let current: Option<&PageRef> = Some(&self.btree.root);
        let current: &PageRef = &self.btree.root;
        let mut pindex: Option<&PageIndex> = None;
        let mut parent_pindex: Option<&PageIndex> = None;
        let mut page: &LayoutPtr<Page>;
        // let current = &mut self.btree.root as *mut PageRef;
        // let mut pindex: *mut PageIndex = ptr::null_mut();
        // let mut parent_pindex: *mut PageIndex = ptr::null_mut();
        let depth: i32 = 2;

        let key = &self.icur.key;

        loop {
            parent_pindex = pindex;
            page = current.page.as_ref().unwrap();

            if !matches!(page.r#type, PageType::Internal) {
                break;
            }

            let page_index = page.get_page_index()?;

            //TODO: append.

            /**
             * Binary search on internal page.
             *  1. primitive key order.
             *  2. custom key order.
             */
            if key.len() <= FP_BTREE_PRIMITIVE_KEY_MAX_LEN {
                let mut l = 1u32;
                let mut r = page_index.entries - 1;

                while r != 0 {
                    let idx = l + r>>1;
                    let cur_ref = unsafe {
                        &**page_index.indexes.add(idx as usize)
                    };

                    r >>= 1;
                }
            }
        }

        Ok(None)
    }

    fn save_cursor_state(&self) -> BtreeCursorState {
        BtreeCursorState::from_cursor(&self.icur)
    }
}