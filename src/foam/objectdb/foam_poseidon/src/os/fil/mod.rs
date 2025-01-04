use std::sync::Arc;

use crate::{global::NO_IMPL, types::*};

pub mod posix;

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

pub trait FileSystem {
    /**
     * Return a list of file name under given directory
     */
    fn ls(&self, dir: &str, prefix: Option<&str>, suffix: Option<&str>) -> Result<Vec<String>, FPErr> {
        Err(NO_IMPL)
    }

    /**
     * Return true if file exits.
     */
    fn exist(&self, name: &str) -> Result<bool, FPErr>{
        Err(NO_IMPL)
    }

    /**
     * Open a handle for a file.
     */
    fn open(&self, name: &str, file_type: FileType, flags: u32) -> Result<Arc<dyn FileHandle>, FPErr> {
        Err(NO_IMPL)
    }

    /**
     * Remove a file.
     */
    fn rm(&self, name: &str, flags: u32) -> Result<(), FPErr> {
        Err(NO_IMPL)
    }

    /**
     * Rename a file
     */
    fn mv(&self, from: &str, to: &str) -> Result<(), FPErr> {
        Err(NO_IMPL)
    }

    /**
     * Return size of file
     */
    fn size(&self, name: &str) -> Result<u64, FPErr> {
        Err(NO_IMPL)
    }
    /**
     * close FileSystem
     */
    fn close(&self) -> Result<(), FPErr> {
        Err(NO_IMPL)
    }
}

pub trait FileHandle {
    /**
     * Get file system.
     */
    fn get_file_system(&self) -> Arc<dyn FileSystem>;

    /**
     * Close a file handle.
     */
    fn close(&self) -> Result<(), FPErr> {
        Err(NO_IMPL)
    }

    /**
     * POSIX only
     */
    fn advise(&self, offset: FileOffset, len: FileSize, advice: i32) -> Result<(), FPErr> {
        Err(NO_IMPL)
    }

    /**
     * Extend the file.
     */
    fn extend(&self, offset: FileOffset) -> Result<(), FPErr> {
        Err(NO_IMPL)
    }

    /**
     * Extend the file.
     */
    fn extend_nolock(&self, offset: FileOffset) -> Result<(), FPErr> {
        Err(NO_IMPL)
    }

    /**
     * Lock/unlock a file.
     * @param lock whether to lock or unlock
     */
    fn lock(&self, lock: bool) -> Result<(), FPErr> {
        Err(NO_IMPL)
    }

    //TODO: mmap interface.
    //TODO: file 

    /**
     * Read from file.
     */
    fn read(&self, offset: FileOffset, len: FileSize) -> Result<FileBuf, FPErr> {
        Err(NO_IMPL)
    }

    /**
     * Return size of file.
     */
    fn size(&self) -> Result<FileSize, FPErr> {
        Err(NO_IMPL)
    }

    /**
     * flush buffered change into file.
     */
    fn sync(&self) -> Result<(), FPErr> {
        Err(NO_IMPL)
    }

    /**
     * flush buffered change into file without wait it complete.
     */
    fn sync_nowait(&self) -> Result<(), FPErr> {
        Err(NO_IMPL)
    }

    /**
     * Truncate file.
     */
    fn truncate(&self, offset: FileOffset) -> Result<FileBuf, FPErr> {
        Err(NO_IMPL)
    }

    /**
     * Write to a file.
     */
    fn write(&self, offset: FileOffset, len: FileSize, buf: FileBuf) -> Result<(), FPErr> {
        Err(NO_IMPL)
    }

}