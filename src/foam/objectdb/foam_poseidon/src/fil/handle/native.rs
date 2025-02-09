#![allow(unused)]

use std::{fs::File, io::{Read, Seek, SeekFrom}, sync::RwLock};

use crate::{error::FP_NO_IMPL, internal::FPResult};

use super::{FilBuf, FilHandle};

struct NativeFilHandle {
    fd: RwLock<File>,
}

impl FilHandle for NativeFilHandle {
    fn read(&self, offset: u64, len: u64) -> FPResult<FilBuf> {
        //TODO: monitor.
        let mut fd = self.fd.write().unwrap();

        let cur_position = FP_IO_ERR_RET!(fd.seek(SeekFrom::Current(0)));

        FP_IO_ERR_RET!(fd.seek(SeekFrom::Start(offset)));
        let mut data = vec![0u8; len as usize];
        //TODO: retry. replace chunk read.
        FP_IO_ERR_RET!(fd.read_exact(&mut data[..]));

        FP_IO_ERR_RET!(fd.seek(SeekFrom::Start(cur_position)));

        Ok(FilBuf{
            data,
            size: len,
        })
    }
}