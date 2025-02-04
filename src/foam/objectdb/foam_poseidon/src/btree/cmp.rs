#![allow(unused)]

use crate::{cursor::{BaseCursor, CURSOR_BOUND_LOWER_INCLUSIVE, CURSOR_BOUND_UPPER_INCLUSIVE}, error::FP_NO_IMPL, types::FPResult, FP_BIT_IST};

use super::btree::{btree_cursor::BtreeCursor, BTreeType};

/**
 * __wt_compare_bounds.
 */
pub(crate) fn key_bounds_check(bt_cursor: &BtreeCursor, upper: bool) -> FPResult<bool> {

    let mut record_number_bound: u64;

    if upper {
        if matches!(bt_cursor.btree.r#type, BTreeType::Row) {

        } else {
            return Err(FP_NO_IMPL);
        }

        if FP_BIT_IST!(bt_cursor.base.flags, CURSOR_BOUND_UPPER_INCLUSIVE) {
            if matches!(bt_cursor.btree.r#type, BTreeType::Row) {

            } else {
                return Err(FP_NO_IMPL);
            }
        } else {
            if matches!(bt_cursor.btree.r#type, BTreeType::Row) {

            } else {
                return Err(FP_NO_IMPL);
            }
        }
    } else {
        if matches!(bt_cursor.btree.r#type, BTreeType::Row) {

        } else {
            return Err(FP_NO_IMPL);
        }

        if FP_BIT_IST!(bt_cursor.base.flags, CURSOR_BOUND_LOWER_INCLUSIVE) {
            if matches!(bt_cursor.btree.r#type, BTreeType::Row) {

            } else {
                return Err(FP_NO_IMPL);
            }
        } else {
            if matches!(bt_cursor.btree.r#type, BTreeType::Row) {

            } else {
                return Err(FP_NO_IMPL);
            }
        }
    }
    Ok(false)
}