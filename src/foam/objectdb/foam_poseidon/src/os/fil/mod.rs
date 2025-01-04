use std::sync::Arc;

use crate::{global::NO_IMPL, types::FpErr};

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
    fn ls(&self, dir: &str, prefix: Option<&str>, suffix: Option<&str>) -> Result<Vec<String>, FpErr> {
        Err(NO_IMPL)
    }

    /**
     * Return true if file exits.
     */
    fn exist(&self, name: &str) -> Result<bool, FpErr>{
        Err(NO_IMPL)
    }

    /**
     * Open a handle for a file.
     */
    fn open(&self, name: &str, file_type: FileType, flags: u32) -> Result<Arc<dyn FileHandle>, FpErr> {
        Err(NO_IMPL)
    }

    /**
     * Remove a file.
     */
    fn rm(&self, name: &str, flags: u32) -> Result<(), FpErr> {
        Err(NO_IMPL)
    }

    /**
     * Rename a file
     */
    fn mv(&self, from: &str, to: &str) -> Result<(), FpErr> {
        Err(NO_IMPL)
    }

    /**
     * Return size of file
     */
    fn size(&self, name: &str) -> Result<u64, FpErr> {
        Err(NO_IMPL)
    }
    /**
     * close FileSystem
     */
    fn close(&self) -> Result<(), FpErr> {
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
    fn close(&self) -> Result<(), FpErr> {
        Err(NO_IMPL)
    }

    /**
     * POSIX only
     */
    fn advise(&self, offset: u64, len: u64, advice: i32) -> Result<(), FpErr> {
        Err(NO_IMPL)
    }

    /**
     * Extend the file.
     */
    fn extend(&self, offset: u64) -> Result<(), FpErr> {
        Err(NO_IMPL)
    }

    /**
     * Extend the file.
     */
    fn extend_nolock(&self, offset: u64) -> Result<(), FpErr> {
        Err(NO_IMPL)
    }

    /**
     * Lock/unlock a file.
     */
    fn lock(&self, offset: u64) -> Result<(), FpErr> {
        Err(NO_IMPL)
    }
}