use crate::error::*;
use super::{file_system, FileType};
use std::sync::{Arc};

struct FileHandle {
    // RefCell
    file_system: Arc<file_system::POSIXFileSystem>,
    file_type: FileType,
    ref_count: std::sync::atomic::AtomicU32,

    name: String,   /* File Name */
    name_hash: u64, /* Hash of File Name */
    last_sync: u64, /* Time of Background fsync */

    //TODO: internal queue
}

impl FileHandle {
    /**
     * Close a file handle.
     */
    fn close(&self) -> Result<(), FpErr> {
        Ok(())
    }

    /**
     * POSIX only
     */
    fn advise(&self, offset: u64, len: u64, advice: i32) -> Result<(), FpErr> {
        Ok(())
    }
}