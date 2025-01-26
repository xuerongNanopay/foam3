#![allow(unused)]

use crate::{btree::{FP_BTREE_MAX_KV_SIZE, FP_RECORD_NUMBER_OOB}, cursor::{CursorFlag, CursorItem, ICursor, CURSOR_BOUND_LOWER, CURSOR_BOUND_LOWER_INCLUSIVE, CURSOR_BOUND_UPPER, CURSOR_BOUND_UPPER_INCLUSIVE}, dao::DAO, error::{FP_NO_IMPL, FP_NO_SUPPORT}, misc::FP_GIGABYTE, types::FPResult, FP_BIT_IS_SET};

use super::{btree_dao::BTreeDAO, BTree, BTreeType, PageRef};

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

/**
 * Btree cursor.
 */
pub(crate) struct BtreeCursor<'a, 'b, 'c> {
    pub(crate) icur: &'a ICursor,
    pub(crate) btree: &'b BTree,
    btree_dao: &'c BTreeDAO<'c>,

    // Date Source.
    pub(crate) data_source: *mut(),

    /* Pointer may be better options */
    pub(crate) cur_page: *mut PageRef,
    pub(crate) slot: u32,
    
    pub(crate) record_number: u64,
}

impl BtreeCursor<'_, '_, '_> {

    /**
     * __wt_btcur_insert
     */
    pub(crate) fn insert(&self) -> FPResult<()> {

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
    fn init(&self, reenter: bool) -> FPResult<()> {
        Ok(())
    }

    /**
     * Reset cursor.
     */
    fn reset(&mut self) -> FPResult<()> {

        Err(FP_NO_IMPL)
    }

    /**
     * Clear position.
     */
    fn clear_position(&mut self) -> FPResult<()> {
        self.record_number = FP_RECORD_NUMBER_OOB;
        Err(FP_NO_IMPL)
    }

    fn save_cursor_state(&self) -> BtreeCursorState {
        BtreeCursorState::from_cursor(&self.icur)
    }
}