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

#[macro_export]
macro_rules! BIT_OP {
    (SET, $value:expr, $mask:expr) => {
        $value | $mask
    };
    (CLEAR, $value:expr, $mask:expr) => {
        $value & !($mask)
    };
    (TOGGLE, $value:expr, $mask:expr) => {
        $value ^ ($mask)
    };
    (CHECK, $value:expr, $mask:expr) => {
        ($value & $mask) != 0
    };
}

#[macro_export]
macro_rules! BIT_SET {
    ($value:expr, $mask:expr) => {
        BIT_OP!(SET, $value, $mask)
    };
}
#[macro_export]
macro_rules! BIT_CLR {
    ($value:expr, $mask:expr) => {
        BIT_OP!(CLEAR, $value, $mask)
    };
}
#[macro_export]
macro_rules! BIT_TLG {
    ($value:expr, $mask:expr) => {
        BIT_OP!(TOGGLE, $value, $mask)
    };
}
#[macro_export]
macro_rules! BIT_CHK {
    ($value:expr, $mask:expr) => {
        BIT_OP!(CHECK, $value, $mask)
    };
}

#[macro_export]
macro_rules! FP_VERBOSE_DEBUG {
    ($value:expr, $mask:expr) => {
        //TODO:
    };
}

#[macro_export]
macro_rules! OS_LINUX {
    ($($item:item)*) => {
        $(
            #[cfg(target_os = "linux")]
            $item
        )*
    };
}

#[macro_export]
macro_rules! OS_MACOS {
    ($($item:item)*) => {
        $(
            #[cfg(target_os = "macos")]
            $item
        )*
    };
}

#[macro_export]
macro_rules! OS_WIN {
    ($($item:item)*) => {
        $(
            #[cfg(target_os = "windows")]
            $item
        )*
    };
}

#[macro_export]
macro_rules! VEC_U8 {
    [$($x:expr),*] => {
        vec![$($x as u8),*]
    };
}

#[macro_export]
macro_rules! REINTERPRET_CAST_BUF {
    ($vec_u8:ident, $type:ty, $offset:expr) => {
        unsafe {
            let p = $vec_u8.as_mut_ptr();
            p.add($offset);
            & *(p as *const $type)
        }
    };
    ($vec_u8:ident, $type:ty) => {
        REINTERPRET_CAST_BUF!($vec_u8, $type, 0)
    };
}

#[macro_export]
macro_rules! REINTERPRET_CAST_BUF_MUT {
    ($vec_u8:ident, $type:ty, $offset:expr) => {
        unsafe {
            let p = $vec_u8.as_mut_ptr();
            p.add($offset);
            &mut *(p as *mut $type)
        }
    };
    ($vec_u8:ident, $type:ty) => {
        REINTERPRET_CAST_BUF_MUT!($vec_u8, $type, 0)
    };
}

#[macro_export]
macro_rules! REINTERPRET_CAST_PTR {
    ($ptr:ident, $type:ty, $offset:expr) => {
        unsafe {
            $ptr.add($offset);
            & *($ptr as *const $type)
        }
    };
    ($ptr:ident, $type:ty) => {
        REINTERPRET_CAST_PTR!($ptr, $type, 0)
    };
}

#[macro_export]
macro_rules! REINTERPRET_CAST_PTR_MUT {
    ($ptr:ident, $type:ty, $offset:expr) => {
        unsafe {
            $ptr.add($offset);
            &mut *($ptr as *mut $type)
        }
    };
    ($ptr:ident, $type:ty) => {
        REINTERPRET_CAST_PTR_MUT!($ptr, $type, 0)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_max_macro() {
        assert_eq!(FP_MIN!(1, 2), 1);
        assert_eq!(FP_MAX!(1, 2), 2);
        assert_eq!(FP_CLAMP!(1, 3, 5), 3);
        assert_eq!(FP_CLAMP!(4, 3, 5), 4);
        assert_eq!(FP_CLAMP!(6, 3, 5), 5);
    }


    #[test]
    fn test_bit_op_macro() {
        assert_eq!(BIT_OP!(SET, 0u32, 0x2u32), 2);
        assert_eq!(BIT_OP!(CLEAR, 0x2u32, 0x2u32), 0);
        assert_eq!(BIT_OP!(TOGGLE, 0u32, 0x2u32), 2);
        assert_eq!(BIT_OP!(TOGGLE, 0x2u32, 0x2u32), 0);
        assert_eq!(BIT_OP!(CHECK, 0u32, 0x2u32), false);
        assert_eq!(BIT_OP!(CHECK, 0x2u32, 0x2u32), true);
        assert_eq!(BIT_SET!(0u32, 0x2u32), 2);
    }
}
