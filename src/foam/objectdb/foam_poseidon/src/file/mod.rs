#![allow(unused)]

use std::collections::HashMap;

use crate::{error::FP_NO_IMPL, internal::FPResult};

pub(crate) trait File {
    fn close(&mut self) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }

    fn read(&self, offset: u64, size: u64) -> FPResult<Vec<u8>> {
        Err(FP_NO_IMPL)
    }

    fn write(&mut self, data: &[u8], offset :u64, size: u64) -> FPResult<()> {
        Err(FP_NO_IMPL)
    } 
}

pub(crate) struct MemFile {
    map: HashMap<(u64, u64), Vec<u8>>
}

impl MemFile {
    pub(crate) fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl File for  MemFile {
    fn close(&mut self) -> FPResult<()> {
        Ok(())
    }

    fn read(&self, offset: u64, size: u64) -> FPResult<Vec<u8>> {
        let v = self.map.get(&(offset, size)).unwrap();
        Ok(v.to_owned())
    }

    fn write(&mut self, data: &[u8], offset :u64, size: u64) -> FPResult<()> {
        self.map.insert((offset, size), data.to_vec()).unwrap();
        Ok(())
    } 
}