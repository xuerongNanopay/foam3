#![allow(unused)]

use super::Block;
use crate::util::hash_city;

fn block_open(filename: &str) -> Result<Block, i32> {
    let block: Block = Default::default();

    let hash = hash_city::city_hash_64(filename, filename.len());
    return Ok(block)
}