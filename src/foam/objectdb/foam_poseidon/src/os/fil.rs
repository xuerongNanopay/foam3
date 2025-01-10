mod posix;
mod native;

use std::{result, sync::Arc};

use native::{DefaultFileHandle, DefaultFileSystem};

use crate::{error::*, types::*};

OS_LINUX! {
    pub type FPFileSystem = DefaultFileSystem;
    pub type FPFileHandle = DefaultFileHandle;
}


OS_MACOS! {
    pub type FPFileSystem = DefaultFileSystem;
    pub type FPFileHandle = DefaultFileHandle;
}


OS_WIN! {
    pub type FPFileSystem = DefaultFileSystem;
    pub type FPFileHandle = DefaultFileHandle;
}


pub static FP_FS_OPEN_ACCESS_RAND:u32 = 0x01u32;
pub static FP_FS_OPEN_ACCESS_SEQ:u32  = 0x02u32;
pub static FP_FS_OPEN_CREATE:u32      = 0x04u32;
pub static FP_FS_OPEN_DURABLE:u32     = 0x08u32;
pub static FP_FS_OPEN_EXCLUSIVE:u32   = 0x10u32;
pub static FP_FS_OPEN_FIXED:u32       = 0x20u32;
pub static FP_FS_OPEN_FORCE_MMAP:u32  = 0x40u32;
pub static FP_FS_OPEN_READONLY:u32    = 0x80u32;

pub enum AccessMode {
    Random,
    Sequential
}

pub enum FileType {
    Checkpoint,
    Data,
    Directory,
    Log,
    Regular,
}

pub struct FileStats {
    total_file_open: i32,
}



pub trait FileSystem {
    type FH;

    /**
     * Return stats of the FileSystem.
     */
    fn stats(&self) -> Option<FileStats> {
        None
    }

    /**
     * Return a list of file name under given directory
     */
    fn ls(&self, dir: &str, prefix: Option<&str>, suffix: Option<&str>) -> FPResult<Vec<String>> {
        Err(FP_NO_IMPL)
    }

    /**
     * Return true if file exits.
     */
    fn exist(&self, name: &str) -> FPResult<bool>{
        Err(FP_NO_IMPL)
    }

    /**
     * Open a handle for a file.
     */
    fn open(&self, name: &str, file_type: FileType, flags: u32) -> FPResult<Arc<Self::FH>> {
        Err(FP_NO_IMPL)
    }

    /**
     * Remove a file.
     */
    fn rm(&self, name: &str, flags: u32) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }

    /**
     * Rename a file
     */
    fn rename(&self, from: &str, to: &str) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }

    /**
     * Return size of file
     */
    fn size(&self, name: &str) -> FPResult<FPFileSize> {
        Err(FP_NO_IMPL)
    }

    /**
     * Drop a file handle.
     */
    fn close_fh(&self, name: &str) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }

    /**
     * close FileSystem
     */
    fn close(&self) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }

}

pub trait FileHandle {
    type FM;

    /**
     * Close a file handle.
     */
    fn close(&self) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }

    /**
     * POSIX only
     */
    fn advise(&self, offset: FPFileOffset, len: FPFileSize, advice: i32) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }

    /**
     * Extend the file.
     */
    fn extend(&self, offset: FPFileOffset) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }

    /**
     * Extend the file.
     */
    fn extend_nolock(&self, offset: FPFileOffset) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }

    /**
     * Lock/unlock a file.
     * @param lock whether to lock or unlock
     */
    fn lock(&self, lock: bool) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }

    //TODO: mmap interface.
    //TODO: file 

    /**
     * Read from file.
     */
    fn read(&self, offset: FPFileOffset, len: FPFileSize) -> FPResult<(FPFileBuf, FPFileSize)>  {
        Err(FP_NO_IMPL)
    }

    /**
     * Return size of file.
     */
    fn size(&self) -> FPResult<FPFileSize> {
        Err(FP_NO_IMPL)
    }

    /**
     * flush buffered change into file.
     */
    fn sync(&self) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }

    /**
     * flush buffered change into file without wait it complete.
     */
    fn sync_nowait(&self) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }

    /**
     * Truncate file.
     */
    fn truncate(&self, offset: FPFileOffset) -> Result<FPFileBuf, FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * Write to a file.
     */
    fn write(&self, offset: FPFileOffset, len: FPFileSize, buffer: &FPFileBuf) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }

}

pub fn open(file_system: Arc<FPFileSystem>, name: &str, file_type: FileType, flags: u32) -> FPResult<Arc<FPFileHandle>> {
    let fh = file_system.open(name, file_type, flags)?;
    Ok(fh)
}

pub fn close(file_system: Arc<FPFileSystem>, name: &str) -> FPResult<()> {
    file_system.close_fh(name)?;
    Ok(())
}