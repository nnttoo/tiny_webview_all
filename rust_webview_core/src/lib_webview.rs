use crate::webview_open;

use std::{
    collections::HashMap,
    ffi::c_int,
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicI32, Ordering},
        mpsc,
    },
    thread,
};

use tao::{
    event::{Event, WindowEvent},
    event_loop::{EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget},
    platform::windows::EventLoopBuilderExtWindows,
    window::{Window, WindowId},
};
use wry::{
    WebView,
    dpi::{LogicalPosition, LogicalSize},
};

use crate::webconfig::WebArg;

type BoxedCommand = Box<dyn FnOnce(&EventLoopWindowTarget<CustomEvent>) + Send + 'static>;
// 2. Enum untuk menampung perintah
pub enum CustomEvent {
    Execute(BoxedCommand),
}

struct UnsafeWrapper<T>(pub T);
unsafe impl<T> Send for UnsafeWrapper<T> {}
unsafe impl<T> Sync for UnsafeWrapper<T> {}

struct WindowStored {
    #[allow(dead_code)] // mematikan warning
    window: Window,
    #[allow(dead_code)] // mematikan warning
    webview: WebView,
    web_arg: *const WebArg,
    window_id: i32,
}

static PROXY: OnceLock<EventLoopProxy<CustomEvent>> = OnceLock::new();
static WEBVIEWS: OnceLock<Mutex<HashMap<WindowId, UnsafeWrapper<WindowStored>>>> = OnceLock::new();

static ID_GENERATOR: AtomicI32 = AtomicI32::new(10);
fn get_id_generator() -> i32 {
    ID_GENERATOR.fetch_add(1, Ordering::SeqCst)
}

fn close_window(window_id: WindowId) {
    // Langsung akses WEBVIEWS, inisialisasi jika belum, lalu lock
    let mut map = WEBVIEWS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();

    let Some(map_value) = map.get(&window_id) else {
        return;
    };

    let UnsafeWrapper(mytupple) = map_value;
    let callback_ptr = mytupple.web_arg;
    unsafe {
        let web_arg: &WebArg = &*callback_ptr;
        (web_arg.on_window_closed)();
    }

    if map.remove(&window_id).is_some() {
        println!("Window {:?} clossed.", window_id);
    }
}
fn save_window(id: WindowId, window: Window, webview: WebView, web_arg: *const WebArg) {
    let Ok(mut map) = WEBVIEWS.get_or_init(|| Mutex::new(HashMap::new())).lock() else {
        return;
    };

    let window_id;

    unsafe {
        let myweb: &WebArg = &*web_arg;
        window_id = myweb.windowid;
    }

    map.insert(
        id,
        UnsafeWrapper(WindowStored {
            window,
            webview,
            web_arg,
            window_id,
        }),
    );

    println!("Window {:?} store to HashMap.", id);
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

    unsafe {
        let webarg: &mut WebArg = &mut *webconfig_mut;
        webarg.windowid = get_id_generator();
    }

    let config_addr = webconfig_mut as usize;

    let command = Box::new(move |elwt: &EventLoopWindowTarget<CustomEvent>| {
        // Logic bikin window...
        unsafe {
            let ptr = config_addr as *mut WebArg;
            let config: &WebArg = &*ptr;
            let (w, bw, w_id) = webview_open::open_webview(config, elwt);
            save_window(w_id, w, bw, config);
        }
    });

    let _ = proxy.send_event(CustomEvent::Execute(command));
}

fn find_window_by_id<F>(windownumber: i32, callback: F)
where
    F: FnOnce(&WindowId, &WindowStored),
{
    {
        let map = WEBVIEWS
            .get_or_init(|| Mutex::new(HashMap::new()))
            .lock()
            .unwrap();
        for (itemkey, item) in map.iter() {
            let UnsafeWrapper(itemunwrap) = item;
            if itemunwrap.window_id == windownumber {
                callback(itemkey, itemunwrap);
                return;
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn closeWindow(windowid: c_int) {
    let window_id_i32: i32 = windowid;
    let Some(proxy) = PROXY.get() else {
        return;
    };
    let _ = proxy.send_event(CustomEvent::Execute(Box::new(
        move |_: &EventLoopWindowTarget<CustomEvent>| {
            let mut windowid: Option<WindowId> = None;

            find_window_by_id(window_id_i32, |key, _| {
                // di sini harus pakai variable karena
                // find_window_by_id itu Mutex lock
                windowid = Some(*key);
            });

            if let Some(windowid_unwrap) = windowid {
                close_window(windowid_unwrap);
            }
        },
    )));
}

#[unsafe(no_mangle)]
pub extern "C" fn moveWindow(windowid: c_int, left: c_int, top: c_int) {
    let window_id_i32: i32 = windowid;
    let Some(proxy) = PROXY.get() else {
        return;
    };

    let _ = proxy.send_event(CustomEvent::Execute(Box::new(
        move |_: &EventLoopWindowTarget<CustomEvent>| {
            find_window_by_id(window_id_i32, |_, mywindow| {
                mywindow
                    .window
                    .set_outer_position(LogicalPosition::new(left, top));
            });
        },
    )));
}

#[unsafe(no_mangle)]
pub extern "C" fn resizeWindow(windowid: c_int, width: c_int, height: c_int) {
    let window_id_i32: i32 = windowid;
    let Some(proxy) = PROXY.get() else {
        return;
    };

    let _ = proxy.send_event(CustomEvent::Execute(Box::new(
        move |_: &EventLoopWindowTarget<CustomEvent>| {
            find_window_by_id(window_id_i32, |_, mywindow| {
                mywindow
                    .window
                    .set_inner_size(LogicalSize::new(width, height));
            });
        },
    )));
}

#[unsafe(no_mangle)]
pub extern "C" fn maximize(windowid: c_int, ismaximize: bool) {
    let window_id_i32: i32 = windowid;
    let Some(proxy) = PROXY.get() else {
        return;
    };

    let _ = proxy.send_event(CustomEvent::Execute(Box::new(
        move |_: &EventLoopWindowTarget<CustomEvent>| {
            find_window_by_id(window_id_i32, |_, mywindow| {
                mywindow.window.set_maximized(ismaximize);
            });
        },
    )));
}

#[unsafe(no_mangle)]
pub extern "C" fn minimize(windowid: c_int, isminimize: bool) {
    let window_id_i32: i32 = windowid;
    let Some(proxy) = PROXY.get() else {
        return;
    };

    let _ = proxy.send_event(CustomEvent::Execute(Box::new(
        move |_: &EventLoopWindowTarget<CustomEvent>| {
            find_window_by_id(window_id_i32, |_, mywindow| {
                mywindow.window.set_minimized(isminimize);
            });
        },
    )));
}
