#![allow(unused)]

static FP_KILOBYTE: u64 = 1024;
static FP_MEGABYTE: u64 = 1048576;
static FP_GIGABYTE: u64 = 1073741824;
static FP_TERABYTE: u64 = 1099511627776;
static FP_PETABYTE: u64 = 1125899906842624;
static FP_EXABYTE:  u64 = 1152921504606846976;

static FP_DAY:    u64 = 86400;
static FP_MINITE: u64 = 60;

#[macro_export]
macro_rules! FP_MIN {
    ($a:expr, $b:expr) => {
        if $a < $b { $a } else { $b }
    };
}

#[macro_export]
macro_rules! FP_MAX {
    ($a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }
    };
}

#[macro_export]
macro_rules! FP_CLAMP {
    ($x:expr, $low:expr, $high:expr) => {
        FP_MIN!(FP_MAX!($x, $low), $high)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_max_micro() {
        assert_eq!(FP_MIN!(1, 2), 1);
        assert_eq!(FP_MAX!(1, 2), 2);
        assert_eq!(FP_CLAMP!(1, 3, 5), 3);
        assert_eq!(FP_CLAMP!(4, 3, 5), 4);
        assert_eq!(FP_CLAMP!(6, 3, 5), 5);
    }
}
