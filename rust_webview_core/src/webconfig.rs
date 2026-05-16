use std::{ffi::{CStr, c_char, c_int}, os::raw::c_void};  

#[repr(C)]
pub struct ResourceRequest {
    pub uri: *const c_char,
    pub method: *const c_char,
    pub body: *const u8,   // unsigned char* -> *mut u8
    pub body_len: usize, // size_t -> usize 
}

#[repr(C)]
pub struct ResourceResponse {
    pub body: *const u8, // unsigned char* -> *mut u8
    pub body_len: usize,
    pub content_type: *mut c_char,
    pub status: c_int,
}

pub type SendResponse = extern "C" fn(response: *const ResourceResponse,*const c_void);

#[repr(C)]
pub struct WebArg {
    pub url: *const c_char, 
    pub title: *const c_char,
    pub custom_protocol: *const c_char,
    pub on_custom_protocol: extern "C" fn(*const ResourceRequest, SendResponse, *const c_void), 
    pub on_window_closed : extern "C" fn(),
    pub width: c_int,
    pub height: c_int,
    pub is_kiosk: bool,
    pub is_maximize: bool,
    pub is_debug: bool,
}

pub fn get_string_from_cpointer(strptr: *const c_char) -> String {
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

pub fn string_tocstring(str : String)->std::ffi::CString{
    return std::ffi::CString::new(str).unwrap();
}