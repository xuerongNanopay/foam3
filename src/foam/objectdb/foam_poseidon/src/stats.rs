#![allow(unused)]

use std::{ptr, sync::{atomic::{AtomicI64, AtomicPtr}, LazyLock}};

// pub static FP_STATS: LazyLock<FPStats> = LazyLock::new(|| {
//     FPStats::default()
// });


macro_rules! FP_STAT_MODEL {
    ($sname:ident, $internalname:ident, $vname:ident, $getfn:ident, $($field_name:ident: $field_type:ty),*) => {
        pub static $vname: std::sync::LazyLock<$internalname> = std::sync::LazyLock::new(|| {
            $internalname::default()
        });
        
        #[derive(Debug, Default)]
        pub struct $internalname {
            $(pub $field_name: std::sync::atomic::AtomicI64),*
        }

        #[derive(Debug, Default, Copy, Clone)]
        pub struct $sname {
            $(pub $field_name: i64),*
        }

        pub fn get_fp_stats() -> $sname {
            $sname {
                $($field_name: $vname.$field_name.load(std::sync::atomic::Ordering::Relaxed)),*
            }
        }
    };
}

FP_STAT_MODEL!(FPStats, FPStatsInternal, FP_STATS, get_fp_stats,
    block_read: AtomicI64,
    block_size: AtomicI64
);

#[macro_export]
macro_rules! FP_STATS_INCR {
    ($p:ident, $v:expr) => {
        crate::stats::FP_STATS.$p.fetch_add($v as i64, std::sync::atomic::Ordering::SeqCst)
    };
    ($p:ident) => {
        FP_STATS_INCR!($p, 1)
    };
}

