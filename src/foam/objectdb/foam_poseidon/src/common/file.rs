#![allow(unused)]

use std::{fmt, fs::File, ops::Range};

use async_trait::async_trait;

use crate::{error::{FP_ILLEGAL_ARGUMENT, FP_NO_IMPL}, internal::FPResult};

use super::{ArcBytes, HasLength};

#[async_trait]
pub(crate) trait FileHandle: 'static + Send + Sync + HasLength + fmt::Debug {
    fn read(&self, range: Range<usize>) -> FPResult<ArcBytes>;

    async fn read_async(&self, range: Range<usize>) -> FPResult<ArcBytes> {
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
    fn read(&self, range: Range<usize>) -> FPResult<ArcBytes> {
        let file_len = self.len();

        let start = range.start;
        let end = FP_MIN!(file_len, range.end);

        if start >= end {
            return Err(FP_ILLEGAL_ARGUMENT);
        }

        let mut buffer = vec![0u8; (end - start) as usize];

        use std::io::{Read, Seek, SeekFrom};
        /* Avoid reset seek */
        let mut file = FP_IO_ERR_RET!(self.file.try_clone());
        FP_IO_ERR_RET!(file.seek(SeekFrom::Start(start as u64)));
        FP_IO_ERR_RET!(file.read_exact(&mut buffer));

        Ok(ArcBytes::new(buffer))
    }
}

impl HasLength for FileWrapper {
    fn len(&self) -> usize {
        self.len as usize
    }
}

#[async_trait]
impl FileHandle for &'static [u8] {
    fn read(&self, range: Range<usize>) -> FPResult<ArcBytes> {
        let bytes = &self[range];
        Ok(ArcBytes::new(bytes))
    }

    async fn read_async(&self, range: Range<usize>) -> FPResult<ArcBytes> {
        Ok(self.read(range)?)
    }
}