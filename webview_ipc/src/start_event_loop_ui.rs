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

struct UiControllerItem {
    pub _window: Window,
    pub _webview: WebView,
    pub _window_id: u32,
}

pub struct UiController {
    hash_map: HashMap<WindowId, UiControllerItem>,
    pub app_ctx : AppMyContextArc
}

impl UiController {
    pub fn new(appctx : AppMyContextArc) -> Self {
        Self {
            hash_map: HashMap::new(),
            app_ctx : appctx
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

    pub fn remove_by_id(&mut self, win_id : u32){

        let id : Option<WindowId> = (||{
            for (key, item) in self.hash_map.iter() {
                if item._window_id == win_id {
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

    pub fn is_window_empty(&self)->bool{
        return self.hash_map.is_empty();
    }

    pub fn remove(&mut self, wi : WindowId){
        self.hash_map.remove(&wi);

        if self.is_window_empty(){
            self.app_ctx.req_stop_all("all windows closed");
        }
        
    } 
}
