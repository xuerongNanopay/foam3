#![allow(unused)]

use std::{fmt, fs::File, ops::Range, sync::Arc};

use async_trait::async_trait;

use crate::{error::{FP_ILLEGAL_ARGUMENT, FP_NO_IMPL}, internal::FPResult};

use super::{ByteSlice, HasLength};

#[async_trait]
pub(crate) trait FileHandle: 'static + Send + Sync + HasLength + fmt::Debug {
    fn read(&self, range: Range<usize>) -> FPResult<ByteSlice>;

    async fn read_async(&self, range: Range<usize>) -> FPResult<ByteSlice> {
        Err(FP_NO_IMPL)
    }
}

#[derive(Debug)]
struct StdFile {
    file: File,
    len: u64,
}

impl StdFile {
    pub(crate) fn new(file: File) -> FPResult<Self> {
        let fd = FP_IO_ERR_RET!(file.metadata());
        let len = fd.len();
        Ok(StdFile {file, len})
    }
}

#[async_trait]
impl FileHandle for StdFile {
    fn read(&self, range: Range<usize>) -> FPResult<ByteSlice> {
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

        Ok(ByteSlice::new(buffer))
    }
}

impl HasLength for StdFile {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len as usize
    }
}

#[async_trait]
impl FileHandle for &'static [u8] {
    fn read(&self, range: Range<usize>) -> FPResult<ByteSlice> {
        let bytes = &self[range];
        Ok(ByteSlice::new(bytes))
    }

    async fn read_async(&self, range: Range<usize>) -> FPResult<ByteSlice> {
        Ok(self.read(range)?)
    }
}

/**
 * Read from underline file handle.
 */
#[derive(Clone)]
pub struct FileSlice {
    file_handle: Arc<dyn FileHandle>,
    range: Range<usize>,
}

impl FileSlice {
    // pub(crate) fn new(file_handle: Arc<dyn FileHandle>) -> Self {

    // }
}

impl fmt::Debug for FileSlice {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileSlice({:?}, {:?})", &self.file_handle, self.range)
    }
}