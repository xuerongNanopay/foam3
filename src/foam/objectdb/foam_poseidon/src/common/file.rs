#![allow(unused)]

use std::{fmt, fs::File, ops::Range};

use crate::internal::FPResult;

use super::ArcBytes;

pub(crate) trait FileHandle: 'static + Send + Sync + fmt::Debug {
    fn read(&self, range: Range<u64>) -> FPResult<ArcBytes>;
}

struct FileWrapper {
    file: File,
    len: u64,
}

impl FileWrapper {
    pub(crate) fn new(file: File) -> FPResult<Self> {
        let fd = FP_IO_ERR_RET!(file.metadata());
        let len = fd.len();
        Ok(FileWrapper {file, len})
    }
}