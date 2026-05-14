use std::{ops::Add, thread};

static MyPointer: Option<usize> = None;

struct MyPtr(*mut String);
unsafe impl Send for MyPtr {}

pub fn test_pointer() {
    let mut str = String::from("INI STRING");

    let wrapped_ptr = MyPtr(&mut str as *mut String);

    println!("CheckNomorPointer : {}", &mut str as *mut String as usize);

    let t = thread::spawn(move || {
        let wrapper = wrapped_ptr;
        unsafe {
            let MyPtr(actual_ptr) = wrapper;
            let   strptr = &mut *actual_ptr;

            *strptr = String::from("halah ahalah");
            println!("PrintDiThread :  {}", strptr);
        }
        println!("ini test lagi")
    });

    t.join();
    println!("LUARTHREAD CheckNomorPointer : {}", &mut str as *mut String as usize);
    println!("LUARTHREAD  {}", str);
}
