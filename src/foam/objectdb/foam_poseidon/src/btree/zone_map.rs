#![allow(unused)]

use crate::internal::{FPTimeStamp, FPTxnId};

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub(crate) struct ZMPage {
    pub(crate) start_ts: FPTimeStamp,
    pub(crate) stop_ts: FPTimeStamp,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub(crate) struct ZMTimeWindow {
    pub(crate) durable_start: FPTimeStamp,
    pub(crate) start: FPTimeStamp,
    pub(crate) txn_start: FPTxnId,

    pub(crate) durable_end: FPTimeStamp,
    pub(crate) end: FPTimeStamp,
    pub(crate) txn_end: FPTxnId,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub(crate) struct ZMTimeAggregate {
    pub(crate) latest_durable_start: FPTimeStamp,
    pub(crate) latest_start: FPTimeStamp,
    pub(crate) latest_txn_start: FPTxnId,

    pub(crate) latest_durable_end: FPTimeStamp,
    pub(crate) latest_end: FPTimeStamp,
    pub(crate) latest_txn_end: FPTxnId,
}