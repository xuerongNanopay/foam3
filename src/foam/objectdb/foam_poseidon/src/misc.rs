#![allow(unused)]

pub static FP_KILOBYTE: usize = 1024;
pub static FP_MEGABYTE: usize = 1048576;
pub static FP_GIGABYTE: usize = 1073741824;
pub static FP_TERABYTE: usize = 1099511627776;
pub static FP_PETABYTE: usize = 1125899906842624;
pub static FP_EXABYTE:  usize = 1152921504606846976;

pub static FP_DAY:    usize = 86400;
pub static FP_MINITE: usize = 60;

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
        $value |= $mask
    };
    (CLEAR, $value:expr, $mask:expr) => {
        $value &= !($mask)
    };
    (MASK, $value:expr, $mask:expr) => {
        $value &= $mask
    };
    (TOGGLE, $value:expr, $mask:expr) => {
        $value ^= ($mask)
    };
    (IS_SET, $value:expr, $mask:expr) => {
        ($value & $mask) != 0
    };
}

#[macro_export]
macro_rules! FP_BIT_SET {
    ($value:expr, $mask:expr) => {
        crate::FP_BIT_OP!(SET, $value, $mask)
    };
}
#[macro_export]
macro_rules! FP_BIT_CLR {
    ($value:expr, $mask:expr) => {
        crate::FP_BIT_OP!(CLEAR, $value, $mask)
    };
}
#[macro_export]
macro_rules! FP_BIT_MSK {
    ($value:expr, $mask:expr) => {
        crate::FP_BIT_OP!(MASK, $value, $mask)
    };
}
#[macro_export]
macro_rules! FP_BIT_TLG {
    ($value:expr, $mask:expr) => {
        crate::FP_BIT_OP!(TOGGLE, $value, $mask)
    };
}
#[macro_export]
macro_rules! FP_BIT_IST {
    ($value:expr, $mask:expr) => {
        crate::FP_BIT_OP!(IS_SET, $value, $mask)
    };
}

#[macro_export]
macro_rules! FP_VERBOSE_DEBUG {
    ($value:expr, $mask:expr) => {
        //TODO:
    };
}

#[macro_export]
macro_rules! FP_OS_LINUX {
    ($($item:item)*) => {
        $(
            #[cfg(target_os = "linux")]
            $item
        )*
    };
}

#[macro_export]
macro_rules! FP_OS_MACOS {
    ($($item:item)*) => {
        $(
            #[cfg(target_os = "macos")]
            $item
        )*
    };
}

#[macro_export]
macro_rules! FP_OS_WIN {
    ($($item:item)*) => {
        $(
            #[cfg(target_os = "windows")]
            $item
        )*
    };
}

#[macro_export]
macro_rules! FP_REINTERPRET_CAST_BUF {
    ($vec_u8:expr, $type:ty, $offset:expr) => {
        unsafe {
            let p = $vec_u8.as_ptr();
            p.add($offset);
            & *(p as *const $type)
        }
    };
    ($vec_u8:expr, $type:ty) => {
        FP_REINTERPRET_CAST_BUF!($vec_u8, $type, 0)
    };
}

#[macro_export]
macro_rules! FP_REINTERPRET_CAST_BUF_MUT {
    ($vec_u8:expr, $type:ty, $offset:expr) => {
        unsafe {
            let p = $vec_u8.as_mut_ptr();
            p.add($offset);
            &mut *(p as *mut $type)
        }
    };
    ($vec_u8:expr, $type:ty) => {
        FP_REINTERPRET_CAST_BUF_MUT!($vec_u8, $type, 0)
    };
}

#[macro_export]
macro_rules! FP_REINTERPRET_CAST_PTR {
    ($ptr:ident, $type:ty, $offset:expr) => {
        unsafe {
            $ptr.add($offset);
            & *($ptr as *const $type)
        }
    };
    ($ptr:ident, $type:ty) => {
        FP_REINTERPRET_CAST_PTR!($ptr, $type, 0)
    };
}

#[macro_export]
macro_rules! FP_REINTERPRET_CAST_PTR_MUT {
    ($ptr:ident, $type:ty, $offset:expr) => {
        unsafe {
            $ptr.add($offset);
            &mut *($ptr as *mut $type)
        }
    };
    ($ptr:ident, $type:ty) => {
        FP_REINTERPRET_CAST_PTR_MUT!($ptr, $type, 0)
    };
}

#[macro_export]
macro_rules! FP_SIZE_OF {
    ($type:ty) => {
        std::mem::size_of::<$type>()
    };
}

#[macro_export]
macro_rules! FP_ALIGN_OF {
    ($type:ty) => {
        std::mem::align_of::<$type>()
    };
}

#[macro_export]
macro_rules! FP_ALLOC {
    ($type:ty) => {{
        let layout = std::alloc::Layout::new::<$type>();
        unsafe {
            let ptr = std::alloc::alloc(layout);
            if ptr.is_null() {
                Err(crate::error::FP_ALLOC_FAIL)
            } else {
                std::ptr::write_bytes(ptr, 0, layout.size());
                let t = ptr as *mut $type;
                let t = Box::from_raw(t);
                Ok((layout, t))
            }
        }
    }};
    ($type:ty, $size: expr) => {{
        let layout = std::alloc::Layout::array::<$type>($size).unwrap();
        unsafe {
            let ptr = std::alloc::alloc(layout);
            if ptr.is_null() {
                Err(crate::error::FP_ALLOC_FAIL)
            } else {
                std::ptr::write_bytes(ptr, 0, layout.size());
                let t = ptr as *mut $type;
                let t = Vec::from_raw_parts(t, $size, $size);
                Ok((layout, t))
            }
        }
    }};
    ($size:expr, $align:expr) => {{
        let layout = std::alloc::Layout::from_size_align($size, $align).unwrap();
        let final_layout = layout.pad_to_align();
        unsafe {
            let ptr = std::alloc::alloc(final_layout);
            if ptr.is_null() {
                Err(crate::error::FP_ALLOC_FAIL)
            } else {
                std::ptr::write_bytes(ptr, 0, layout.size());
                Ok((final_layout, ptr))
            }
        }
    }};
    ($size:expr) => {
        FP_ALLOC!($size, 8)
    };
    {$ft:ty: $fv:expr, $(,)? $($t:ty: $v:expr),* $(,)?} => {
        FP_ALLOC!(
            $ft, $fv,
            {
                if $fv > 1 {
                    std::alloc::Layout::array::<$ft>($fv).unwrap()
                } else {
                    std::alloc::Layout::new::<$ft>()
                }
            },
            0usize,
            $($t, $v, 
            {
                if $v > 1 {
                    std::alloc::Layout::array::<$t>($v).unwrap()
                } else {
                    std::alloc::Layout::new::<$t>()
                }
            },
            0usize
            ),*
        )
    };
    ($ft:ty, $fv:expr, $fl:expr, $fo:expr, $($t:ty, $v:expr, $l:expr, $o:expr),*) => {{
        let mut combined_layout: std::alloc::Layout = $fl;
        let mut i = 1usize;
        let mut offsets: Vec<usize> = vec![$fo, $($o),*];
        $(
            //TODO: wrap into custom error.
            let (c, o) = combined_layout.extend($l).unwrap();
            combined_layout = c;
            offsets[i] = o;
            i += 1;
        )*
        let combined_layout = combined_layout.pad_to_align();

        unsafe {
            let ptr = std::alloc::alloc(combined_layout);
            if ptr.is_null() {
                Err(crate::error::FP_ALLOC_FAIL)
            } else {
                std::ptr::write_bytes(ptr, 0, combined_layout.size());

                // println!("{:?}", offsets);
                let mut i = 0usize;
                Ok(unsafe {
                    (
                        combined_layout,
                        {
                            let r = ptr.add(offsets[i]) as *mut $ft;
                            i += 1;
                            r
                        },
                        $({
                            let r = ptr.add(offsets[i]) as *mut $t;
                            i += 1;
                            r
                        }),*
                    )
                })
            }
        }
    }};
}

#[macro_export]
macro_rules! FP_DEALLOC {
    ($ptr:expr, $layout:expr) => {
        unsafe {
            std::alloc::dealloc($ptr as *mut u8, $layout);
            $ptr = std::ptr::null_mut();
        }
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
        crate::BIT_REVERSE!(16, $v)
    }
}

#[macro_export]
macro_rules! BIT_REVERSE_32 {
    ($v:expr) => {
        crate::BIT_REVERSE!(32, $v)
    }
}

#[macro_export]
macro_rules! BIT_REVERSE_64 {
    ($v:expr) => {
        crate::BIT_REVERSE!(64, $v)
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
        // assert_eq!(FP_BIT_OP!(SET, 0u32, 0x2u32), 2);
        // assert_eq!(FP_BIT_OP!(CLEAR, 0x2u32, 0x2u32), 0);
        // assert_eq!(FP_BIT_OP!(TOGGLE, 0u32, 0x2u32), 2);
        // assert_eq!(FP_BIT_OP!(TOGGLE, 0x2u32, 0x2u32), 0);
        // assert_eq!(FP_BIT_OP!(IS_SET, 0u32, 0x2u32), false);
        // assert_eq!(FP_BIT_OP!(IS_SET, 0x2u32, 0x2u32), true);
        // assert_eq!(FP_BIT_SET!(0u32, 0x2u32), 2);
    }

    #[test]
    fn test_bit_reverse_macro() {
        assert_eq!(BIT_REVERSE_16!(0x1122), 0x2211);
        assert_eq!(BIT_REVERSE_32!(0x11223344), 0x44332211);
        assert_eq!(BIT_REVERSE_64!(0x1122334411223344), 0x4433221144332211);
        let a = 8;
    }
}
