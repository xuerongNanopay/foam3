#![allow(unused)]

use crate::{block::FP_BLOCK_HEADER_LEN, FP_MIN};

mod btree;
mod row;
mod page;

pub static FP_BTREE_MAX_KV_SIZE: usize = i32::MAX as usize - 1024;
const FP_PAGE_HEADER_LEN: usize = 30;
const FP_RESERVE_HEADER_LEN: usize = FP_PAGE_HEADER_LEN + FP_BLOCK_HEADER_LEN;

const FP_RECORD_NUMBER_OOB: u64 = 0; /* Illegal record number */

const FP_BTREE_INSERT_SKIP_MAX_DEPTH: usize = 10;
const FP_BTREE_INSERT_SKIP_PROBABILITY: u32 = u32::MAX >> 2;

const FP_BTREE_LEX_PREFIX_CMP_MAX_LEN: usize = 9;

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

/**
 * Lexicographic comparison for prefix.
 * Expect prefix length is less than FP_BTREE_LEX_PREFIX_CMP_MAX_LEN.
 * search_key > tree_key => positive
 * search_key = tree_key => 0
 * search_key < tree_key => negative
 */
fn lex_prefix_cmp(search_key: (*const u8, usize), tree_key: (*const u8, usize)) -> i32 {
    let len = FP_MIN!(search_key.1, tree_key.1);

    let mut s_key = search_key.0;
    let mut t_key = tree_key.0;

    unsafe {
        for _ in 0..len {
            if *s_key != *t_key {
                return (*s_key - *t_key) as i32
            }
            s_key = s_key.add(1);
            t_key = t_key.add(1);
        }
    }
    if search_key.1 == tree_key.1 {
        0
    } else {
        (search_key.1 - tree_key.1) as i32
    }
}

/**
 * Lexicographic comparison.
 * Allow compare start at skip_len offset in the array.
 * Use in compare large key duing btree traversing.
 */
fn lex_skip_cmp(search_key: (*const u8, usize), tree_key: (*const u8, usize), skip_len: usize) -> i32 {
    1
}