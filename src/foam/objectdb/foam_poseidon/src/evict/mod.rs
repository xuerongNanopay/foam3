#![allow(unused)]

pub(crate) type EvictCallFlag = u32;

pub(crate) const FP_EVICT_CALL_NO_SPLIT: EvictCallFlag   = 1 << 1; /* do not try to split when read. */
pub(crate) const FP_EVICT_CALL_URGENT:   EvictCallFlag   = 1 << 2; /* urgent eviction. */