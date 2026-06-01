use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
};

use tao::window::{Window, WindowId};
use wry::WebView;

use crate::app_ctx::AppMyContextArc;

static ID_GENERATOR: AtomicU32 = AtomicU32::new(10);
fn get_id_generator() -> u32 {
    ID_GENERATOR.fetch_add(1, Ordering::Relaxed)
}

pub struct UiControllerItem {
    pub window: Window,
    pub _webview: WebView,
    pub window_id: u32,
}

pub struct UiController {
    hash_map: HashMap<WindowId, UiControllerItem>,
    pub app_ctx: AppMyContextArc,
}

impl UiController {
    pub fn new(appctx: AppMyContextArc) -> Self {
        Self {
            hash_map: HashMap::new(),
            app_ctx: appctx,
        }
    }

    pub fn add(&mut self, w: Window, web: WebView) -> u32 {
        let wid = get_id_generator();
        let widd = w.id();

        let item = UiControllerItem {
            window_id: wid,
            _webview: web,
            window: w,
        };

        self.hash_map.insert(widd, item);

        wid
    }

    pub fn remove_by_id(&mut self, win_id: u32) {
        let id: Option<WindowId> = (|| {
            for (key, item) in self.hash_map.iter() {
                if item.window_id == win_id {
                    return Some(key.clone());
                }
            }

            return None;
        })();

        let Some(id) = id else {
            return;
        };

        self.remove(id);
    }

    pub fn is_window_empty(&self) -> bool {
        return self.hash_map.is_empty();
    }

    pub fn remove(&mut self, wi: WindowId) {
        self.hash_map.remove(&wi);

        if self.is_window_empty() {
            self.app_ctx.req_stop_all("all windows closed");
        }
    }

    pub fn get_window_byid(&self, wi: u32) -> Result<&UiControllerItem, &str> {
        for item in self.hash_map.iter() {
            if item.1.window_id == wi {
                return Ok(&item.1);
            }
        }

        Err("cant get window")
    }
}
