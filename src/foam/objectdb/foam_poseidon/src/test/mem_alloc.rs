#![allow(unused)]

#[cfg(test)]
mod tests {

    #[repr(C)]
    struct AAA {
        pub i: u8,
        pub j: u32,
    }

    #[repr(align(16))]
    struct BBB {
        pub i: u8,
        pub j: u64,
    }

    #[repr(C)]
    struct CCC {
        pub i: bool,
        pub j: u64,
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
    fn test_alloc_a_buffer() {
        let (l, p) = FP_ALLOC!(13, 16).unwrap();
        assert_eq!((p as usize)%16, 0);
        let (l, p) = FP_ALLOC!(13).unwrap();
        assert_eq!((p as usize)%8, 0);
    }

    #[test]
    fn test_alloc_single_type() {
        {
            let (_, mut t) = FP_ALLOC!(AAA).unwrap();
            t.i = 10;
            assert_eq!(t.i, 10);
            let (_, mut t)  = FP_ALLOC!(AAA, 2).unwrap();
            t[1].i = 20;
            assert_eq!(t[1].i, 20);
        }
    }

    #[test]
    fn test_alloc_different_type() {
        let (layout, mut pa1, p2) = FP_ALLOC!{
            AAA: 1,
            BBB: 2,
            // CCC: 3,
        }.unwrap();
        
        {
            //Illegal, combine layout should use dealloc.
            // let bpa1 = unsafe {
            //     Box::from_raw(pa1)   
            // };
            // let bpa1 = unsafe {
            //     Box::from_raw(pc1)   
            // };
        }

        FP_DEALLOC!(pa1, layout);

        let a = FP_ALLOC![AAA];
        // double alloc.
        // FP_DEALLOC!(pa1, layout);
        // let (layout, pa2, pb2, pc2) = FP_ALLOC![AAA, BBB, CCC];
    }
}