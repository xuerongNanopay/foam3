#![allow(unused)]

#[macro_export]
macro_rules! LOG_INFO {
    ($($arg:tt)*) => {
        println!("file: {} | line: {} | message: {}", line!(), file!(), format!($($arg)*));   
    };
}

#[macro_export]
macro_rules! LOG_ERR {
    ($($arg:tt)*) => {
        println!("file: {} | line: {} | message: {}", line!(), file!(), format!($($arg)*));   
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        LOG_INFO!("hello, {} -- {}", "World", "Xuerong")
    }
}
