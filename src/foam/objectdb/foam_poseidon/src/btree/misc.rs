#![allow(unused)]
static FP_KILOBYTE: u64 = 1024;
static FP_MEGABYTE: u64 = 1048576;
static FP_GIGABYTE: u64 = 1073741824;
static FP_TERABYTE: u64 = 1099511627776;
static FP_PETABYTE: u64 = 1125899906842624;
static FP_EXABYTE:  u64 = 1152921504606846976;

static FP_DAY:    u64 = 86400;
static FP_MINITE: u64 = 60;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn btree() {
        // let result = add_numbers(2, 2);
        // assert_eq!(result, 4);
    }
}
