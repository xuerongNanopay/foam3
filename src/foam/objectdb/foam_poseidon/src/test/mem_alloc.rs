#![allow(unused)]

#[cfg(test)]
mod tests {

    #[repr(C)]
    struct AAA {
        i: u8,
        j: u32,
    }

    #[repr(align(16))]
    struct BBB {
        i: u8,
        j: u64,
    }

    #[repr(C)]
    struct CCC {
        i: bool,
        j: u64,
    }


    #[test]
    fn test_alignment() {
        assert_eq!(FP_ALIGN_OF!(AAA), 4);
        assert_eq!(FP_ALIGN_OF!(BBB), 16);
        assert_eq!(FP_ALIGN_OF!(CCC), 8);
    }

    #[test]
    fn test_size() {
        assert_eq!(FP_SIZE_OF!(AAA), 8);
        // Interesting.
        assert_eq!(FP_SIZE_OF!(BBB), 16);
        assert_eq!(FP_SIZE_OF!(CCC), 16);
    }

    #[test]
    fn test_alloc_mutliply_struct() {
        
    }
}