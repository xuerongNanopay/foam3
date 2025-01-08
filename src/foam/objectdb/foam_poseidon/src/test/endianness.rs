#![allow(unused)]

#[cfg(test)]
mod tests {

    #[repr(C)]
    union EndianTest {
        value: u32,
        bytes: [u8; 4],
    }

    #[test]
    fn demo_what_is_the_endianness() {
        let test = EndianTest { value: 0x01020304 };

        unsafe {
            match test.bytes[0] {
                0x01 => println!("System is big-endian"),
                0x04 => println!("System is little-endian"),
                _ => println!("Unknown endianness"),
            }
        }
    }
}