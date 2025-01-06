use std::sync::Arc;

use crate::{error::FP_NO_IMPL, types::*};

pub mod posix;
pub mod native;


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
    type FH;

    // TODO: generic stats.
    // fn open_file_count(&self) -> usize;

    /**
     * Return a list of file name under given directory
     */
    fn ls(&self, dir: &str, prefix: Option<&str>, suffix: Option<&str>) -> Result<Vec<String>, FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * Return true if file exits.
     */
    fn exist(&self, name: &str) -> Result<bool, FPErr>{
        Err(FP_NO_IMPL)
    }

    /**
     * Open a handle for a file.
     */
    fn open(&self, name: &str, file_type: FileType, flags: u32) -> Result<Arc<Self::FH>, FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * Remove a file.
     */
    fn rm(&self, name: &str, flags: u32) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * Rename a file
     */
    fn rename(&self, from: &str, to: &str) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * Return size of file
     */
    fn size(&self, name: &str) -> Result<FPFileSize, FPErr> {
        Err(FP_NO_IMPL)
    }
    /**
     * close FileSystem
     */
    fn close(&self) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }
}

pub trait FileHandle {
    /**
     * Close a file handle.
     */
    fn close(&self) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * POSIX only
     */
    fn advise(&self, offset: FPFileOffset, len: FPFileSize, advice: i32) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * Extend the file.
     */
    fn extend(&self, offset: FPFileOffset) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * Extend the file.
     */
    fn extend_nolock(&self, offset: FPFileOffset) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * Lock/unlock a file.
     * @param lock whether to lock or unlock
     */
    fn lock(&self, lock: bool) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }

    //TODO: mmap interface.
    //TODO: file 

    /**
     * Read from file.
     */
    fn read(&self, offset: FPFileOffset, len: FPFileSize) -> Result<(FPFileBuf, FPFileSize), FPErr>  {
        Err(FP_NO_IMPL)
    }

    /**
     * Return size of file.
     */
    fn size(&self) -> Result<FPFileSize, FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * flush buffered change into file.
     */
    fn sync(&self) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * flush buffered change into file without wait it complete.
     */
    fn sync_nowait(&self) -> Result<(), FPErr> {
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
    fn write(&self, offset: FPFileOffset, len: FPFileSize, buffer: &FPFileBuf) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }

}