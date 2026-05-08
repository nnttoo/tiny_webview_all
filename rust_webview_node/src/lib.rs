use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(C)]
pub struct WebArg {
    pub url: *const c_char,
    pub mycb: extern "C" fn(),
}

#[unsafe(no_mangle)]
pub extern "C" fn hello_world(webconfig_mut: *mut WebArg) {
    let webconfig = unsafe { &mut *webconfig_mut };
    let url_str = unsafe {
        if !webconfig.url.is_null() {
            CStr::from_ptr(webconfig.url)
                .to_str()
                .unwrap_or("Invalid UTF-8")
        } else {
            ""
        }
    };

    println!("Menjalankan: {}", url_str);

    webconfig.url = "coba inidulu\n\0".as_ptr() as *const i8;
    println!("berhasil mengubah");

    // 2. Menjalankan callback
    (webconfig.mycb)();
}

#[unsafe(no_mangle)]
pub extern "C" fn tambah(a: i32, b: i32) -> i32 {
    a + b
}
