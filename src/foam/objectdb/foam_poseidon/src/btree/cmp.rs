#![allow(unused)]

use crate::{cursor::BaseCursor, types::FPResult};

use super::btree::{btree_cursor::BtreeCursor, BTreeType};

pub(crate) fn key_bounds_check(bt_cursor: &BtreeCursor, upper: bool) -> FPResult<bool> {

    if upper {
        if matches!(bt_cursor.btree.r#type, BTreeType::Row) {
            
        }
    } else {

    }
    Ok(false)
}