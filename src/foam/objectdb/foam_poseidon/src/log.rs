#![allow(unused)]

#[macro_export]
macro_rules! FP_LOG_INFO {
    ($($arg:tt)*) => {
        println!("file: {} | line: {} | message: {}", line!(), file!(), format!($($arg)*));   
    };
}

#[macro_export]
macro_rules! FP_LOG_ERR {
    ($($arg:tt)*) => {
        println!("file: {} | line: {} | message: {}", line!(), file!(), format!($($arg)*));   
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        FP_LOG_ERR!("hello, {} -- {}", "World", "Xuerong")
    }
}
