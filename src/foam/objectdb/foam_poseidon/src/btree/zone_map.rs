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
    pub(crate) start_commit_at: FPTimeStamp,
    pub(crate) start_at: FPTimeStamp,
    pub(crate) start_by: FPTxnId,

    pub(crate) end_commit_at: FPTimeStamp,
    pub(crate) end_at: FPTimeStamp,
    pub(crate) end_by: FPTxnId,

    pub(crate) in_txn_prepare: u8,
}

impl ZMTxnValue {
    pub(crate) fn new() -> ZMTxnValue {
        ZMTxnValue {
            start_commit_at   : FP_TIME_STAMP_MIN,
            start_at          : FP_TIME_STAMP_MIN,
            start_by          : FP_TXN_ID_NONE,
            end_commit_at    : FP_TIME_STAMP_MIN,
            end_at           : FP_TIME_STAMP_MAX,
            end_by           : FP_TXN_ID_MAX,
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
    pub(crate) newest_start_commit_at: FPTimeStamp,
    pub(crate) newest_end_commit_at: FPTimeStamp,

    pub(crate) oldest_start_at: FPTimeStamp,
    pub(crate) newest_mod_by: FPTxnId,

    pub(crate) newest_end_at: FPTimeStamp,
    pub(crate) newest_end_by: FPTimeStamp,

    pub(crate) in_txn_prepare: u8,
}

impl ZMTxnAddr {
    #[inline(always)]
    pub(crate) fn new() -> ZMTxnAddr {
        ZMTxnAddr {
            newest_start_commit_at  : FP_TIME_STAMP_NONE,
            newest_end_commit_at    : FP_TIME_STAMP_NONE,
            oldest_start_at         : FP_TIME_STAMP_NONE,
            newest_mod_by           : FP_TXN_ID_NONE,
            newest_end_at           : FP_TIME_STAMP_MAX,
            newest_end_by           : FP_TXN_ID_MAX,
            in_txn_prepare          : 0u8,
        }
    }
}