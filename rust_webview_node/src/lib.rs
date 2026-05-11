mod webconfig;
mod webview_open;

use std::{os::raw::c_void, thread};

use crate::webconfig::{ResourceRequest, ResourceResponse, SendResponse, WebArg, get_string_from_cpointer};

type MyCallback = extern "C" fn(progress: i32);
#[unsafe(no_mangle)]
pub extern "C" fn TestCallback(cb: MyCallback) {
    // Memanggil callback dari sisi Rust
    // Pastikan callback tidak null jika dikirim dari bahasa lain
    cb(50);
    cb(100);
}

#[unsafe(no_mangle)]
pub extern "C" fn openWebView(webconfig_mut: *mut WebArg) {
    if webconfig_mut.is_null() {
        return;
    }

    println!("Rust: Test Callback start");
    let boxed =  webconfig_mut as usize;  
    std::thread::spawn(move || { 

        unsafe { 
            let ptr = boxed as *mut WebArg;
            let config = &mut *ptr;
            println!("{}",  get_string_from_cpointer(config.title));
            webview_open::open_webview(config);
        }
    });
 
}
