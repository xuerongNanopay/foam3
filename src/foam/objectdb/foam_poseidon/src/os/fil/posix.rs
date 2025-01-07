use super::{FileHandle, FileSystem};

pub struct PosixFileSystem {
}

impl FileSystem for PosixFileSystem {
    type FH = PosixFileHandle;
}

pub struct PosixFileHandle {

}

impl FileHandle for PosixFileHandle {
    type FM = PosixFileSystem;
}