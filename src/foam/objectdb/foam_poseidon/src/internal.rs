#![allow(unused)]

pub static FP_KILOBYTE: u64 = 1024;
pub static FP_MEGABYTE: u64 = 1048576;
pub static FP_GIGABYTE: u64 = 1073741824;
pub static FP_TERABYTE: u64 = 1099511627776;
pub static FP_PETABYTE: u64 = 1125899906842624;
pub static FP_EXABYTE:  u64 = 1152921504606846976;

pub static FP_DAY:    u64 = 86400;
pub static FP_MINITE: u64 = 60;

pub(crate) type FPTimeStamp = u64;
pub(crate) const FP_TIME_STAMP_NONE: FPTimeStamp = 0;
pub(crate) const FP_TIME_STAMP_MIN:  FPTimeStamp = 1;
pub(crate) const FP_TIME_STAMP_MAX:  FPTimeStamp = u64::MAX;

pub(crate) type FPTxnId = u64;
pub(crate) const FP_TXN_ID_NONE:    FPTxnId = 0;
pub(crate) const FP_TXN_ID_MIN:     FPTxnId = 1;
pub(crate) const FP_TXN_ID_MAX:     FPTxnId = u64::MAX - 10;
pub(crate) const FP_TXN_ID_ABORTED: FPTxnId = u64::MAX;

use std::{collections::HashMap, sync::{Arc, Mutex, RwLock}, u64};

pub type FPErr = i32;
pub type FPResult<T> = Result<T, FPErr>;


pub type FPConcurrentHashMap<K, V> = RwLock<HashMap<K, V>>;

pub(crate) type FPRwLock<T> = RwLock<T>;
pub(crate) type FPMutex<T> = Mutex<T>;
pub(crate) type FPSpin<T> = Mutex<T>;

#[macro_export]
macro_rules! FP_INFO {
    ($($arg:tt)*) => {
        println!("file: {} | line: {} | message: {}", file!(), line!(), format!($($arg)*));   
    };
}

#[macro_export]
macro_rules! FP_ERROR {
    ($($arg:tt)*) => {
        println!("file: {} | line: {} | message: {}", file!(), line!(), format!($($arg)*))   
    };
}

