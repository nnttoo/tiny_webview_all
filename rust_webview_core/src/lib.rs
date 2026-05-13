mod webconfig;
mod webview_open;
 
use std::{
    collections::HashMap, ffi::c_char, sync::{Mutex, OnceLock, mpsc}, thread
};

use rfd::FileDialog;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget},
    platform::windows::EventLoopBuilderExtWindows,
    window::{Window, WindowId},
};
use wry::WebView;

use crate::webconfig::{WebArg, get_string_from_cpointer};

type BoxedCommand = Box<dyn FnOnce(&EventLoopWindowTarget<CustomEvent>) + Send + 'static>;
// 2. Enum untuk menampung perintah
enum CustomEvent {
    Execute(BoxedCommand),
}
 

struct UnsafeWrapper<T>(pub T);
unsafe impl<T> Send for UnsafeWrapper<T> {}
unsafe impl<T> Sync for UnsafeWrapper<T> {}


static PROXY: OnceLock<EventLoopProxy<CustomEvent>> = OnceLock::new();
static WEBVIEWS: OnceLock<
    Mutex<HashMap<WindowId, UnsafeWrapper<(Window, WebView, *const WebArg)>>>,
> = OnceLock::new();

fn close_window(window_id: WindowId) {
    // Langsung akses WEBVIEWS, inisialisasi jika belum, lalu lock
    let mut map = WEBVIEWS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();

    if let Some(wrapper) = map.get(&window_id) {
        let (_, _, arg_ptr) = &wrapper.0;

        unsafe {
            let cbwraper: &WebArg = &**arg_ptr;
            (cbwraper.on_window_closed)();
        }
    }

    if map.remove(&window_id).is_some() {
        println!("Window {:?} ditutup.", window_id);
    }
}
fn save_window(id: WindowId, window: Window, webview: WebView, cb_wraper: *const WebArg) {
    if let Ok(mut map) = WEBVIEWS.get_or_init(|| Mutex::new(HashMap::new())).lock() {
        map.insert(id, UnsafeWrapper((window, webview, cb_wraper)));

        println!("Window {:?} sukses disimpan di brankas.", id);
    }
}

fn create_event_loop() {
    if PROXY.get().is_some() {
        return;
    }

    println!("event loop created");
    let (tx, rx) = mpsc::channel::<EventLoopProxy<CustomEvent>>();
    thread::spawn(move || {
        let event_loop = EventLoopBuilder::<CustomEvent>::with_user_event()
            .with_any_thread(true)
            .build();

        let proxy = event_loop.create_proxy();

        tx.send(proxy).unwrap();

        event_loop.run(move |event, elwt, _| match event {
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

    create_event_loop();

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
            let (w, bw, w_id) = webview_open::open_webview(config, elwt); 
            save_window(w_id, w, bw, config);
        }
    });

    let _ = proxy.send_event(CustomEvent::Execute(command));
    println!("Event berhasil dikirim, PROXY masih aman di brankas."); 
}

#[unsafe(no_mangle)]
pub extern "C" fn get_active_window_count() -> usize {
    if let Ok(map) = WEBVIEWS.get_or_init(|| Mutex::new(HashMap::new())).lock() {
        let count = map.len();
        count
    } else {
        0 // Jika gagal lock, anggap 0 (aman)
    }
}



#[unsafe(no_mangle)]
pub extern  "C" fn select_file(
    file_type_c : *const c_char, 
    file_ext_c : *const c_char
){

    let file_type = get_string_from_cpointer(file_type_c);
    let file_ext = get_string_from_cpointer(file_ext_c);

    let exts : Vec<String> = file_ext.split(",")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    let file = FileDialog::new()
        .add_filter(file_type, &exts)  
        .set_directory("/")              
        .pick_file();    

    match file {
        Some(path) => println!("User memilih: {:?}", path),
        None => println!("User membatalkan pilihan."),
    }
}