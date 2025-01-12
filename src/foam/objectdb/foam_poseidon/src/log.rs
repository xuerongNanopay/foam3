#![allow(unused)]

#[macro_export]
macro_rules! INFO {
    ($($arg:tt)*) => {
        println!("file: {} | line: {} | message: {}", line!(), file!(), format!($($arg)*));   
    };
}

#[macro_export]
macro_rules! ERROR {
    ($($arg:tt)*) => {
        println!("file: {} | line: {} | message: {}", line!(), file!(), format!($($arg)*));   
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        INFO!("hello, {} -- {}", "World", "Xuerong")
    }
}
