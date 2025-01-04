mod btree;
#[macro_use]
mod misc;
mod block;
mod os;
mod session;
mod util;
mod error;

#[no_mangle]
pub extern "C" fn Java_foam_poseidon_Poseidon_addNumbers(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
    a: i32,
    b: i32,
) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn Java_foam_poseidon_Poseidon_mulNumbers(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
    a: i32,
    b: i32,
) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add_numbers(2, 2);
        // assert_eq!(result, 4);
    }
}
