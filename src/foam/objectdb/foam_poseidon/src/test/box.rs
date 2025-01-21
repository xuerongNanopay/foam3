#![allow(unused)]

#[cfg(test)]
mod tests {

    #[test]
    fn test_box() {
        struct A {
            a: i32,
        }

        let b = Box::new(A{a: 1});
        let c = &*b;
        let d: *const A = c;

        // Illegal
        // {
        //     let e = *b;
        // }

        // let d = b;
    }
}