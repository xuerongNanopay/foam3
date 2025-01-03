#![allow(unused)]

use super::Block;

fn block_open() -> Result<Block, i32> {
    let block: Block = Default::default();
    return Ok(block)
}