#![allow(unused)]

use crate::{btree::FP_BTREE_MAX_KV_SIZE, cursor::{BaseCursor, CursorFlag, Item, CURSOR_BOUND_LOWER, CURSOR_BOUND_UPPER}, dao::DAO, error::FP_NO_SUPPORT, misc::FP_GIGABYTE, types::FPResult, FP_BIT_IS_SET};

use super::{btree_dao::BTreeDAO, BTree, BTreeType, PageRef};

/**
 * Btree cursor.
 */
pub(crate) struct BtreeCursor<'a, 'b, 'c> {
    pub(crate) base: &'a BaseCursor,
    pub(crate) btree: &'b BTree,
    btree_dao: &'c BTreeDAO<'c>,

    // Date Source.
    pub(crate) data_source: *mut(),

    /* Pointer may be better options */
    pub(crate) cur_page: *mut PageRef,
    pub(crate) slot: u32,
    
}

#[derive(Default)]
struct CursorState {
    record_number: u64,
    flags: CursorFlag
}

impl CursorState {
    fn from_cursor(cursor: &BaseCursor) -> CursorState {
        CursorState::default()
    }
}

impl BtreeCursor<'_, '_, '_> {

    /**
     * __wt_btcur_insert
     */
    pub(crate) fn insert(&self) -> FPResult<()> {

        let mut save_state: CursorState = CursorState::default();

        //TODO: stats
        let insert_len = self.base.key.len() + self.base.value.len();

        // Verify KV length.
        if matches!(self.btree.r#type, BTreeType::Row) {
            self.size_check(&self.base.key)?;
        }
        self.size_check(&self.base.value)?;

        // Bulk-load only available before the first insert.
        self.btree_dao.get_btree().disable_bulk_load();

        //TODO: support append for column stored.

        //TODO: save cursor state.


        Ok(())
    }

    /**
     * Check if a insert item is too large. 
     */
    fn size_check(&self, item: &Item) -> FPResult<()> {

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
    fn is_key_in_bounds(&self) -> FPResult<bool> {

        /* Ignore if not config */
        if FP_BIT_IS_SET!(self.base.flags, CURSOR_BOUND_LOWER | CURSOR_BOUND_UPPER){
            return Ok(false);
        }

        //TODO: unlikely assert.

        if FP_BIT_IS_SET!(self.base.flags, CURSOR_BOUND_LOWER) && false {

        }

        if FP_BIT_IS_SET!(self.base.flags, CURSOR_BOUND_UPPER) {
            
        }

        Ok(false)
    }

}