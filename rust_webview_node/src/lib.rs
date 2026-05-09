use std::ffi::{CStr, CString, c_int};
use std::os::raw::c_char;
mod webview_open;

#[repr(C)]
pub struct WebArg {
    pub url: *const c_char,
    pub wclassname: *const c_char,
    pub title: *const c_char,
    pub custom_protocol: *const c_char,
    pub on_custom_protocol: extern "C" fn(*const c_char),
    pub width: c_int,
    pub height: c_int,
    pub is_kiosk: bool,
    pub is_maximize: bool,
    pub is_debug: bool,
}

fn get_string_from_cpointer(strptr: *const c_char) -> String {
    let result = unsafe {
        if !strptr.is_null() {
            CStr::from_ptr(strptr)
                .to_str()
                .unwrap_or("Invalid UTF-8")
                .to_string()
        } else {
            String::new()
        }
    };

    return result;
}

#[unsafe(no_mangle)]
pub extern "C" fn openWebView(webconfig_mut: *mut WebArg) {
    if webconfig_mut.is_null() {
        return;
    }

    let webconfig = unsafe { &mut *webconfig_mut };

    let navtive_webconfig = webview_open::WebViewConfig {
        wclassname: get_string_from_cpointer(webconfig.wclassname),
        url: get_string_from_cpointer(webconfig.url),
        custom_protocol: get_string_from_cpointer(webconfig.custom_protocol),
        title: get_string_from_cpointer(webconfig.title),
        height: webconfig.height as i32,
        width: webconfig.width as i32,
        is_debug: webconfig.is_debug,
        is_kiosk: webconfig.is_kiosk,
        is_maximize: webconfig.is_maximize,
        on_custom_protocol: Box::new(|mystr: &str| {
            println!("{}", mystr);
        }),
    };

    (webconfig.on_custom_protocol)(
        CString::new("test aja dong").unwrap().as_ptr()
    );

    webview_open::open_webview(&navtive_webconfig);
}

#[unsafe(no_mangle)]
pub extern "C" fn tambah(a: i32, b: i32) -> i32 {
    a + b
}
