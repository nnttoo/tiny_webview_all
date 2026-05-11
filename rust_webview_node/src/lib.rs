mod webconfig;
mod webview_open;

use std::{os::raw::c_void, thread};

use crate::webconfig::{ResourceRequest, ResourceResponse, SendResponse, WebArg};

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
    let webconfig = unsafe { &mut *webconfig_mut };
    let on_custom_protocol = webconfig.on_custom_protocol;

 
        unsafe {
            let test_req = ResourceRequest {
                uri: std::ptr::null(),
                method: std::ptr::null(),
                body: std::ptr::null(),
                body_len: 0,
            };
            extern "C" fn my_callback(response: *const ResourceResponse, userdata: *const c_void) {
                println!("dipanggil dari rust");
            }

            println!("Rust: Mencoba memanggil callback untuk testing...");
            let cb: SendResponse = my_callback;
            // 3. Panggil callback
            (on_custom_protocol)(
                &test_req as *const ResourceRequest,
                cb,
                std::ptr::null(),
            );
        } 

    //webview_open::open_webview(&webconfig);
}
