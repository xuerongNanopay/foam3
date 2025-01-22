/**
 * Fixed-length column-store leaf page.
 */
#[repr(C)]
 struct PageColFix {
    fix_bitf: u8,
}

/**
 * Variable-length column-store leaf page.
 */
#[repr(C)]
 struct PageColVar {

}