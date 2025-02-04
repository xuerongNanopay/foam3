#![allow(unused)]

pub(crate) type FPTimeStamp = u64;
pub(crate) const FP_TIME_STAMP_NONE: FPTimeStamp = 0;
pub(crate) const FP_TIME_STAMP_MIN:  FPTimeStamp = 1;
pub(crate) const FP_TIME_STAMP_MAX:  FPTimeStamp = u64::MAX;

pub(crate) type FPTxnId = u64;
pub(crate) const FP_TXN_ID_NONE:    FPTxnId = 0;
pub(crate) const FP_TXN_ID_MIN:     FPTxnId = 1;
pub(crate) const FP_TXN_ID_MAX:     FPTxnId = u64::MAX - 10;
pub(crate) const FP_TXN_ID_ABORTED: FPTxnId = u64::MAX;

use std::{collections::HashMap, sync::{Arc, RwLock}, u64};

pub type FPErr = i32;
pub type FPResult<T> = Result<T, FPErr>;

pub type FPFileSize = u64;
pub type FPFileBuf = Vec<u8>;

pub type FPConcurrentHashMap<K, V> = RwLock<HashMap<K, V>>;