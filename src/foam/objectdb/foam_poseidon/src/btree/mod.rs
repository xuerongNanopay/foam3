#![allow(unused)]

use crate::block::FP_BLOCK_HEADER_LEN;

mod btree;
mod row;
mod page;

pub static FP_BTREE_MAX_KV_SIZE: usize = i32::MAX as usize - 1024;
const FP_PAGE_HEADER_LEN: usize = 30;
const FP_RESERVE_HEADER_LEN: usize = FP_PAGE_HEADER_LEN + FP_BLOCK_HEADER_LEN;

const FP_RECORD_NUMBER_OOB: u64 = 0; /* Illegal record number */

const FP_BTREE_INSERT_SKIP_MAX_DEPTH: usize = 10;
const FP_BTREE_INSERT_SKIP_PROBABILITY: u32 = u32::MAX >> 2;

const FP_BTREE_PRIMITIVE_KEY_MAX_LEN: usize = 9;

pub(crate) struct BtreeInsert {
    record_number: u64,
    offset: u32, /* row-store key data start */
    size: u32, /* row-store key data size */
}

/**
 * The skiplist of BtreeInsert.
 */
pub(crate) struct BtreeInsertList {
    head: [*mut BtreeInsert; FP_BTREE_INSERT_SKIP_MAX_DEPTH],
    tail: [*mut BtreeInsert; FP_BTREE_INSERT_SKIP_MAX_DEPTH],
}

fn lex_compare_short(s_p: *const u8, s_s: usize, d_p: *const u8, d_s: usize) {

}