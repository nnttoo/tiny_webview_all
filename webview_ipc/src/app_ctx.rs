use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicI32, AtomicU32, Ordering},
    },
};

use rfd::MessageDialogResult::No;
use tao::{
    event_loop::{EventLoopProxy, EventLoopWindowTarget},
    window::{Window, WindowId},
};
use wry::WebView;

type BoxedCommand = Box<dyn FnOnce(&EventLoopWindowTarget<CustomEvent>) + Send + 'static>;

type MyWindowMap = HashMap<WindowId, UnsafeWindowMap>;
struct UnsafeWindowMap {
    pub _window: Window,
    pub _webview: WebView,
    pub _window_id: u32,
}

unsafe impl Send for UnsafeWindowMap {}
unsafe impl Sync for UnsafeWindowMap {}

impl UnsafeWindowMap {
    pub fn new(window: Window, webview: WebView) -> Self {
        Self {
            _webview: webview,
            _window: window,
            _window_id: get_id_generator(),
        }
    }
}

type MyWindowMapMutexArc = Arc<Mutex<MyWindowMap>>;

pub enum CustomEvent {
    Execute(BoxedCommand),
}

static ID_GENERATOR: AtomicU32 = AtomicU32::new(10);
fn get_id_generator() -> u32 {
    ID_GENERATOR.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone)]
pub struct AppMyContext {
    hash_map: MyWindowMapMutexArc,
    pub even_loop_poxy: Arc<EventLoopProxy<CustomEvent>>,
}

impl AppMyContext {
    pub fn new(event_loop: EventLoopProxy<CustomEvent>) -> Self {
        Self {
            hash_map: Arc::new(Mutex::new(HashMap::new())),
            even_loop_poxy: Arc::new(event_loop),
        }
    }
    pub fn webview_add(&mut self, webview: WebView, window: Window) -> u32 {
        let Ok(mut hash_map) = self.hash_map.try_lock() else {
            return 0;
        };

        let window_id = window.id();
        let myunsafe = UnsafeWindowMap::new(window, webview);
        let wind_32id = myunsafe._window_id;
        hash_map.insert(window_id, myunsafe);

        wind_32id
    }

    pub fn webview_remove(&mut self, windowid: WindowId) -> bool {
        let Ok(mut hash_map) = self.hash_map.try_lock() else {
            return false;
        };

        hash_map.remove(&windowid);
        let isempty = hash_map.is_empty();
        isempty
    }

    pub fn webview_close(&mut self, wiid: u32) {
        let windowid: Option<WindowId> = match self.hash_map.try_lock() {
            Ok(hashmap) => {
                let mut found_key = None;
                for (key, item) in hashmap.iter() {
                    if item._window_id == wiid {
                        found_key = Some(key.clone());
                        break;
                    }
                }
                found_key
            }
            _ => None,
        };

        match windowid {
            Some(windowid) => {
                self.webview_remove(windowid);
            }
            _ => {}
        }
    }
}
