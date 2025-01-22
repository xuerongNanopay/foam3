#![allow(unused)]

use super::PageRef;

/**
 * 
 */
pub(super) struct BtreeCursor {
    /* Pointer may be better options */
    pub(super) cur_page: *mut PageRef,
    pub(super) slot: u32,
    
}