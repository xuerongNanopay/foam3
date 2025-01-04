use super::{file_system, FileType};
use std::sync::{Arc};

struct FileHandle {
    file_system: Arc<file_system::POSIXFileSystem>,
    file_type: FileType,
    ref_count: std::sync::atomic::AtomicU32,

    name: String,   /* File Name */
    name_hash: u64, /* Hash of File Name */
    last_sync: u64, /* Time of Background fsync */

    //TODO: internal queuee
}