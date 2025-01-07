#![allow(unused)]

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::ptr;
    #[derive(Debug)]
    struct Foo {
        pub name: String,
        pub value: i32,
    }

    #[test]
    fn demo_atomic_ptr_swap() {
        let a_cfg: AtomicPtr<Foo> =  AtomicPtr::new(ptr::null_mut());
        let f = a_cfg.load(Ordering::SeqCst);
        unsafe {
            //This will throw null error.
            //println!("{}", f.read_volatile().name);
        }

        let new_config = Box::new(Foo {
            name: "Demo_Config".to_string(),
            value: 42,
        });

        let fp = Box::into_raw(new_config);
        a_cfg.store(fp, Ordering::SeqCst);
        let f = a_cfg.load(Ordering::SeqCst);
        unsafe {
            // println!("{}", f.read_volatile().name);
            println!("{}", (*fp).value);
            println!("{}", (*f).value);
        }

        // release memory
        unsafe {
            Box::from_raw(fp);
        }

        // throww error as null pointer
        // unsafe {
        //     println!("{}", (*fp).name);
        //     println!("{}", (*f).name);
        // }

        // May not you expect.
        unsafe {
            if fp.is_null() {
                println!("Yes, I am null.");
            } else {
                println!("No, I am not null.");
            }
            if fp.is_aligned() {
                println!("Yes, I am aligned.");
            } else {
                println!("No, I am not aligned.");
            }
        }
    }
}
