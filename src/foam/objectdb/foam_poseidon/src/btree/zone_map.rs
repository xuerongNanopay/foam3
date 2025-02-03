#![allow(unused)]

use crate::internal::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct ZMPage {
    pub(crate) start_ts: FPTimeStamp,
    pub(crate) stop_ts: FPTimeStamp,
}

/**
 * Metadata for transaction use by value(leaf node value)
 */
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct ZMTxnValue {
    pub(crate) init_commit_at: FPTimeStamp,
    pub(crate) init_at: FPTimeStamp,
    pub(crate) init_by: FPTxnId,

    pub(crate) del_commit_at: FPTimeStamp,
    pub(crate) del_at: FPTimeStamp,
    pub(crate) del_by: FPTxnId,

    pub(crate) in_txn_prepare: u8,
}

impl ZMTxnValue {
    pub(crate) fn new() -> ZMTxnValue {
        ZMTxnValue {
            init_commit_at   : FP_TIME_STAMP_MIN,
            init_at          : FP_TIME_STAMP_MIN,
            init_by          : FP_TXN_ID_NONE,
            del_commit_at    : FP_TIME_STAMP_MIN,
            del_at           : FP_TIME_STAMP_MAX,
            del_by           : FP_TXN_ID_MAX,
            in_txn_prepare   : 0u8,
        }
    }
}

/**
 * Zone map to record transaction metadata for address(value of internal node)
 */
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct ZMTxnAddr {
    pub(crate) newest_init_commit_at: FPTimeStamp,
    pub(crate) newest_del_commit_at: FPTimeStamp,

    pub(crate) oldest_init_at: FPTimeStamp,
    pub(crate) newest_by: FPTxnId,

    pub(crate) newest_del_at: FPTimeStamp,
    pub(crate) newest_del_by: FPTimeStamp,

    pub(crate) in_txn_prepare: u8,
}

impl ZMTxnAddr {
    #[inline(always)]
    pub(crate) fn new() -> ZMTxnAddr {
        ZMTxnAddr {
            newest_init_commit_at   : FP_TIME_STAMP_MIN,
            newest_del_commit_at    : FP_TIME_STAMP_MIN,
            oldest_init_at          : FP_TIME_STAMP_MIN,
            newest_by               : FP_TXN_ID_NONE,
            newest_del_at           : FP_TIME_STAMP_MAX,
            newest_del_by           : FP_TXN_ID_NONE,
            in_txn_prepare          : 0u8,
        }
    }
}