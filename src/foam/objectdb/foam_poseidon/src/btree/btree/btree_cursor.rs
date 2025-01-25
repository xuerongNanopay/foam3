#![allow(unused)]

use crate::{btree::FP_BTREE_MAX_KV_SIZE, cursor::{BaseCursor, Item}, dao::DAO, error::FP_NO_SUPPORT, misc::FP_GIGABYTE, types::FPResult};

use super::{btree_dao::BTreeDAO, BTree, BTreeType, PageRef};

/**
 * Btree cursor.
 */
pub(super) struct BtreeCursor<'a, 'b, 'c> {
    base: &'a BaseCursor,
    btree: &'b BTree,
    btree_dao: &'c BTreeDAO<'c>,

    // Date Source.
    pub(super) data_source: *mut(),

    /* Pointer may be better options */
    pub(super) cur_page: *mut PageRef,
    pub(super) slot: u32,
    
    flags: u32,
}


impl BtreeCursor<'_, '_, '_> {
    /**
     * __wt_btcur_insert
     */
    pub(super) fn insert(&self) -> FPResult<()> {
        //TODO: stats
        let insert_len = self.base.key.len() + self.base.value.len();

        // Verify KV length.
        if matches!(self.btree.r#type, BTreeType::Row) {
            self.size_check(&self.base.key)?;
        }
        self.size_check(&self.base.value)?;

        // Bulk-load only available before the first insert.
        self.btree_dao.get_btree().disable_bulk_load();


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
}