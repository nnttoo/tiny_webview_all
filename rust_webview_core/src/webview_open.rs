use crate::lib_webview::CustomEvent;
use crate::webconfig::{
    self, ResourceRequest, ResourceResponse, SendResponse, get_string_from_cpointer,
    string_tocstring,
}; 

use tao::event_loop::EventLoopWindowTarget;
use tao::window::Window;
use tao::window::WindowId;
use tao::window::WindowBuilder;
use wry::{PermissionResponse, WebView, WebViewBuilder, WebViewBuilderExtWindows};
pub fn open_webview(
    webviewcon: &webconfig::WebArg,
    even_loop  :   &EventLoopWindowTarget<CustomEvent>
) ->(Window,WebView, WindowId) { 
    unsafe {
        let mut data_path = std::env::temp_dir();
        data_path.push("webview_lib_data");
        let _ = std::fs::create_dir_all(&data_path);
        std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", data_path.to_str().unwrap());
    }

    let url = get_string_from_cpointer(webviewcon.url); 
    let title = get_string_from_cpointer(webviewcon.title);
    let custom_protocol = get_string_from_cpointer(webviewcon.custom_protocol);

    let mut _builder = WindowBuilder::new()
        .with_title(title.as_str())
        
        .with_inner_size(tao::dpi::LogicalSize::new(
            webviewcon.width,
            webviewcon.height,
        ));

    if webviewcon.is_kiosk {
        _builder = _builder.with_decorations(false);
    };

    if webviewcon.is_maximize {
        _builder = _builder.with_maximized(true);
    }

    let _mywindow = _builder.build(even_loop).unwrap();

    let wid = _mywindow.id();

    let on_custom_protocol = webviewcon.on_custom_protocol;
    let webview = WebViewBuilder::new()
        .with_devtools(webviewcon.is_debug)
        .with_autoplay(true)
        .with_https_scheme(true)
        .with_permission_handler(|kind| {
            println!("Otomatis mengizinkan: {:?}", kind);
            PermissionResponse::Allow
        })
        .with_asynchronous_custom_protocol(custom_protocol, move |_id, _request, responder| {
            let req_method = _request.method().to_string();
            let req_method_cstr = string_tocstring(req_method);
            let uri = _request.uri().to_string();


            let uriptr = string_tocstring(uri); 
            let body_bytes: &[u8] = _request.body();
            let res_req = ResourceRequest {
                uri: uriptr.as_ptr(),
                body: body_bytes.as_ptr(),
                body_len: body_bytes.len(),
                method: req_method_cstr.as_ptr(),
            };

            let myres_callback: SendResponse = {
                extern "C" fn internal_callback(
                    responsess: *const ResourceResponse,
                    user_data: *const std::ffi::c_void,
                ) {
                    unsafe {
                        let Some(phpresponse) = responsess.as_ref() else {
                            return;
                        };

                        let data_slice =
                            std::slice::from_raw_parts(phpresponse.body, phpresponse.body_len);
                        let body_vec = data_slice.to_vec();

                        let response = wry::http::Response::builder()
                            .header(
                                "Content-Type",
                                get_string_from_cpointer(phpresponse.content_type),
                            )
                            .body(body_vec)
                            .unwrap();

                        let responder = Box::from_raw(user_data as *mut wry::RequestAsyncResponder);

                        responder.respond(response);
                    }
                }
                internal_callback
            };
            let responder_ptr = Box::into_raw(Box::new(responder)); 
            on_custom_protocol(&res_req, myres_callback, responder_ptr as *const _); 
        })
        .with_url(url)
        .build(&_mywindow); 
    (
        _mywindow,
        webview.unwrap(),
        wid
    ) 
}
