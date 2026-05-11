mod webconfig;
mod webview_open;
 

use std::{sync::{OnceLock, mpsc}, thread};

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget},
    platform::windows::EventLoopBuilderExtWindows,
};

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

static PROXY: OnceLock<EventLoopProxy<CustomEvent>> = OnceLock::new();

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
                ..
            } => {
                // Jika mau aplikasi mati total saat satu window ditutup:
                *control_flow = ControlFlow::Exit;
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
            webview_open::open_webview(config, elwt);
        }
    });

    let _ = proxy.send_event(CustomEvent::Execute(command));
    println!("Event berhasil dikirim, PROXY masih aman di brankas.");
}
