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
macro_rules! FP_BIT_OP {
    (SET, $value:expr, $mask:expr) => {
        $value | $mask
    };
    (CLEAR, $value:expr, $mask:expr) => {
        $value & !($mask)
    };
    (TOGGLE, $value:expr, $mask:expr) => {
        $value ^ ($mask)
    };
    (IS_SET, $value:expr, $mask:expr) => {
        ($value & $mask) != 0
    };
}

#[macro_export]
macro_rules! FP_BIT_SET {
    ($value:expr, $mask:expr) => {
        FP_BIT_OP!(SET, $value, $mask)
    };
}
#[macro_export]
macro_rules! FP_BIT_CLR {
    ($value:expr, $mask:expr) => {
        FP_BIT_OP!(CLEAR, $value, $mask)
    };
}
#[macro_export]
macro_rules! FP_BIT_TLG {
    ($value:expr, $mask:expr) => {
        FP_BIT_OP!(TOGGLE, $value, $mask)
    };
}
#[macro_export]
macro_rules! FP_BIT_IS_SET {
    ($value:expr, $mask:expr) => {
        FP_BIT_OP!(IS_SET, $value, $mask)
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

#[macro_export]
macro_rules! SIZE_OF {
    ($type:ty) => {
        std::mem::size_of::<$type>()
    };
}

#[macro_export]
macro_rules! BIT_REVERSE {
    (64, $v:expr) => {
        (($v << 56) & 0xff00000000000000u64) | (($v << 40) & 0x00ff000000000000u64) |
        (($v << 24) & 0x0000ff0000000000u64) | (($v << 8) & 0x000000ff00000000u64) |
        (($v >> 8) & 0x00000000ff000000u64) | (($v >> 24) & 0x0000000000ff0000u64) |
        (($v >> 40) & 0x000000000000ff00u64) | (($v >> 56) & 0x00000000000000ffu64)
    };
    (32, $v:expr) => {
        (($v << 24) & 0xff000000u32) | (($v << 8) & 0x00ff0000u32) | 
        (($v >> 8) & 0x0000ff00u32) | (($v >> 24) & 0x000000ffu32)
    };
    (16, $v:expr) => {
        (($v << 8) & 0xff00u16) | (($v >> 8) & 0x00ffu16)
    }
}

#[macro_export]
macro_rules! BIT_REVERSE_16 {
    ($v:expr) => {
        BIT_REVERSE!(16, $v)
    }
}

#[macro_export]
macro_rules! BIT_REVERSE_32 {
    ($v:expr) => {
        BIT_REVERSE!(32, $v)
    }
}

#[macro_export]
macro_rules! BIT_REVERSE_64 {
    ($v:expr) => {
        BIT_REVERSE!(64, $v)
    }
}

#[macro_export]
macro_rules! VEC_U8 {
    [$($x:expr),*] => {
        vec![$($x as u8),*]
    };
    ($s:expr, $f:expr) => {
        let buf = Vec::<u8>::with_capacity($s);
        unsafe { w_buf.set_len($s); }
        buf.fill(0);
        buf
    };
    ($s:expr) => {
        VEC_U8!($s, 0)
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
        assert_eq!(FP_BIT_OP!(SET, 0u32, 0x2u32), 2);
        assert_eq!(FP_BIT_OP!(CLEAR, 0x2u32, 0x2u32), 0);
        assert_eq!(FP_BIT_OP!(TOGGLE, 0u32, 0x2u32), 2);
        assert_eq!(FP_BIT_OP!(TOGGLE, 0x2u32, 0x2u32), 0);
        assert_eq!(FP_BIT_OP!(IS_SET, 0u32, 0x2u32), false);
        assert_eq!(FP_BIT_OP!(IS_SET, 0x2u32, 0x2u32), true);
        assert_eq!(FP_BIT_SET!(0u32, 0x2u32), 2);
    }

    #[test]
    fn test_bit_reverse_macro() {
        assert_eq!(BIT_REVERSE_16!(0x1122), 0x2211);
        assert_eq!(BIT_REVERSE_32!(0x11223344), 0x44332211);
        assert_eq!(BIT_REVERSE_64!(0x1122334411223344), 0x4433221144332211);
        let a = 8;
    }
}
