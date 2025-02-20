#![allow(unused)]

use std::{fmt, fs::File, ops::Range};

use async_trait::async_trait;

use crate::{error::FP_NO_IMPL, internal::FPResult};

use super::ArcBytes;

#[async_trait]
pub(crate) trait FileHandle: 'static + Send + Sync + fmt::Debug {
    fn read(&self, range: Range<u64>) -> FPResult<ArcBytes>;

    async fn read_async(&self, range: Range<u64>) -> FPResult<ArcBytes> {
        Err(FP_NO_IMPL)
    }
}

#[derive(Debug)]
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

#[async_trait]
impl FileHandle for FileWrapper {
    fn read(&self, range: Range<u64>) -> FPResult<ArcBytes> {

    }
}