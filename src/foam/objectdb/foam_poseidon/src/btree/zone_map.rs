#![allow(unused)]

use crate::internal::*;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub(crate) struct ZMPage {
    pub(crate) start_ts: FPTimeStamp,
    pub(crate) stop_ts: FPTimeStamp,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct ZMTimeWindow {
    pub(crate) durable_start: FPTimeStamp,
    pub(crate) start: FPTimeStamp,
    pub(crate) txn_start: FPTxnId,

    pub(crate) durable_end: FPTimeStamp,
    pub(crate) end: FPTimeStamp,
    pub(crate) txn_end: FPTxnId,
}

impl ZMTimeWindow {
    fn new() -> ZMTimeWindow {
        ZMTimeWindow {
            durable_start : FP_TIME_STAMP_MIN,
            start         : FP_TIME_STAMP_MIN,
            txn_start     : FP_TXN_ID_NONE,
            durable_end   : FP_TIME_STAMP_MIN,
            end           : FP_TIME_STAMP_MAX,
            txn_end       : FP_TXN_ID_MAX,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct ZMTimeAggregate {
    pub(crate) latest_durable_start: FPTimeStamp,
    pub(crate) latest_start: FPTimeStamp,
    pub(crate) latest_txn_start: FPTxnId,

    pub(crate) latest_durable_end: FPTimeStamp,
    pub(crate) latest_end: FPTimeStamp,
    pub(crate) latest_txn_end: FPTxnId,
}

impl ZMTimeAggregate {
    #[inline(always)]
    fn new() -> ZMTimeAggregate {
        ZMTimeAggregate {
            latest_durable_start  : FP_TIME_STAMP_MIN,
            latest_start          : FP_TIME_STAMP_MIN,
            latest_txn_start      : FP_TXN_ID_NONE,
            latest_durable_end    : FP_TIME_STAMP_MIN,
            latest_end            : FP_TIME_STAMP_MAX,
            latest_txn_end        : FP_TXN_ID_MAX,
        }
    }
}