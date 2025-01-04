use crate::error::FpErr;

use super::*;
pub trait FileSystem {
    /**
     * Return a list of file name under given directory
     */
    fn file_ls(dir: &str, prefix: Option<&str>, suffix: Option<&str>) 
        -> Result<Vec<String>, FpErr>;

    /**
     * Return true if file exits.
     */
    fn file_exist(name: &str) -> Result<bool, u32>;

    /**
     * Open a handle for a file.
     */
    fn file_open(name: &str, file_type: FileType, flags: u32);

    /**
     * Remove a file.
     */
    fn file_rm(name: &str, flags: u32);

    /**
     * Rename a file
     */
    fn file_mv(from: &str, to: &str);

    /**
     * Return size of file
     */
    fn file_size(name: &str);

    /**
     * close FileSystem
     */
    fn close();
}

pub struct POSIXFileSystem {

}

// impl FileSystem for POSIXFileSystem {

// }