#![allow(unused)]

use crate::types::FPResult;

use super::PageRef;

/**
 * Btree cursor.
 */
pub(super) struct BtreeCursor {
    // Date Source.
    pub(super) data_source: *mut(),

    /* Pointer may be better options */
    pub(super) cur_page: *mut PageRef,
    pub(super) slot: u32,
    
    flags: u32,
}


impl BtreeCursor {
    pub(super) fn insert(&self) -> FPResult<()> {
        Ok(())
    }
}

pub(super) struct BtreeInsert {

}