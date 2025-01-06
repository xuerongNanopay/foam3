#![allow(unused)]

use std::sync::Arc;

use crate::types::FPFileSystem;


struct Global {
    file_system: Option<Arc<FPFileSystem>>,
}



