mod webconfig;
mod webview_open;

use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock, mpsc},
    thread,
};

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget},
    platform::windows::EventLoopBuilderExtWindows,
    window::{Window, WindowId},
};
use wry::WebView;

use crate::webconfig::{
    ResourceRequest, ResourceResponse, SendResponse, WebArg, get_string_from_cpointer,
};

type MyCallback = extern "C" fn(progress: i32);
#[unsafe(no_mangle)]
pub extern "C" fn TestCallback(cb: MyCallback) {
    // Memanggil callback dari sisi Rust
    // Pastikan callback tidak null jika dikirim dari bahasa lain
    cb(50);
    cb(100);
}

type BoxedCommand = Box<dyn FnOnce(&EventLoopWindowTarget<CustomEvent>) + Send + 'static>;
// 2. Enum untuk menampung perintah
enum CustomEvent {
    Execute(BoxedCommand),
}
// 1. Buat Wrapper
struct UnsafeWrapper<T>(pub T);

// 2. Paksa Rust percaya kalau ini aman (karena kita pakai Mutex nanti)
unsafe impl<T> Send for UnsafeWrapper<T> {}
unsafe impl<T> Sync for UnsafeWrapper<T> {}
static PROXY: OnceLock<EventLoopProxy<CustomEvent>> = OnceLock::new();
static WEBVIEWS: OnceLock<Mutex<HashMap<WindowId, UnsafeWrapper<(Window, WebView)>>>> =
    OnceLock::new();

fn close_window(window_id: WindowId) {
    // Langsung akses WEBVIEWS, inisialisasi jika belum, lalu lock
    let mut map = WEBVIEWS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();

    // Langsung hapus
    if map.remove(&window_id).is_some() {
        println!("Window {:?} ditutup.", window_id);
    }
}
fn save_window(id: WindowId, window: Window, webview: WebView) { 
    if let Ok(mut map) = WEBVIEWS.get_or_init(|| Mutex::new(HashMap::new())).lock() { 
        map.insert(id, UnsafeWrapper((window, webview)));
        
        println!("Window {:?} sukses disimpan di brankas.", id);
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn createEventLoop() {
    println!("event loop created");
    let (tx, rx) = mpsc::channel::<EventLoopProxy<CustomEvent>>();
    thread::spawn(move || {
        let event_loop = EventLoopBuilder::<CustomEvent>::with_user_event()
            .with_any_thread(true)
            .build();

        let proxy = event_loop.create_proxy();

        tx.send(proxy).unwrap();

        event_loop.run(move |event, elwt, control_flow| match event {
            Event::UserEvent(CustomEvent::Execute(f)) => {
                f(elwt);
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
                ..
            } => {
                close_window(window_id);
            }

            _ => (),
        });
    });

    let proxy = rx.recv().unwrap();
    let _ = PROXY.set(proxy);
}

#[unsafe(no_mangle)]
pub extern "C" fn openWebView(webconfig_mut: *mut WebArg) {
    if webconfig_mut.is_null() {
        return;
    }

    // Gak perlu 'unsafe' blok di sini!
    let Some(proxy) = PROXY.get() else {
        return;
    };

    let config_addr = webconfig_mut as usize;

    let command = Box::new(move |elwt: &EventLoopWindowTarget<CustomEvent>| {
        // Logic bikin window...
        println!("Membuka window di thread Event Loop...");
        unsafe {
            let ptr = config_addr as *mut WebArg;
            let config: &WebArg = &*ptr;
            let (w,bw,w_id)= webview_open::open_webview(config, elwt); 
            save_window(w_id,w,bw);
        }
    });

    let _ = proxy.send_event(CustomEvent::Execute(command));
    println!("Event berhasil dikirim, PROXY masih aman di brankas.");
}
