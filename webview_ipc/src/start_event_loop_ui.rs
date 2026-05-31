use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
};

use tao::window::{Window, WindowId};
use wry::WebView;

static ID_GENERATOR: AtomicU32 = AtomicU32::new(10);
fn get_id_generator() -> u32 {
    ID_GENERATOR.fetch_add(1, Ordering::Relaxed)
}

struct UiControllerItem {
    pub _window: Window,
    pub _webview: WebView,
    pub _window_id: u32,
}

pub struct UiController {
    hash_map: HashMap<WindowId, UiControllerItem>,
}

impl UiController {
    pub fn new() -> Self {
        Self {
            hash_map: HashMap::new(),
        }
    }

    pub fn add(&mut self, w: Window, web: WebView) -> u32 {
        let wid = get_id_generator();
        let widd = w.id();

        let item = UiControllerItem {
            _window_id: wid,
            _webview: web,
            _window: w,
        };

        self.hash_map.insert(widd, item);

        wid
    }

    pub fn remove(&mut self, wi : WindowId)->bool{
        self.hash_map.remove(&wi);

        self.hash_map.is_empty() 
        
    } 
}
