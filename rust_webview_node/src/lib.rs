mod webconfig;
mod webview_open;

use crate::webconfig::{ResourceRequest, SendResponse, WebArg};


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

    unsafe {
        let test_req = ResourceRequest {
            uri: webconfig.url,
            method: std::ptr::null(),
            body: std::ptr::null(),
            body_len: 0,
        };
        let null_callback: SendResponse = std::mem::transmute(std::ptr::null::<std::ffi::c_void>());

        println!("Rust: Mencoba memanggil callback untuk testing...");

        // 3. Panggil callback
        (webconfig.on_custom_protocol)(
            &test_req as *const ResourceRequest,
            null_callback,
            std::ptr::null(),
        );
    }

    //webview_open::open_webview(&webconfig);
}
#[unsafe(no_mangle)]
pub extern "C" fn tambah(a: i32, b: i32) -> i32 {
    a + b
}
