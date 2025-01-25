#![allow(unused)]

use crate::block::FP_BLOCK_HEADER_LEN;

mod btree;
mod row;
mod page;

pub static FP_BTREE_MAX_KV_SIZE: usize = i32::MAX as usize - 1024;
const FP_PAGE_HEADER_LEN: usize = 30;
const FP_RESERVE_HEADER_LEN: usize = FP_PAGE_HEADER_LEN + FP_BLOCK_HEADER_LEN;

