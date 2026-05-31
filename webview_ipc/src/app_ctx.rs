use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU32, Ordering},
    },
};

use tao::{
    event_loop::{EventLoopProxy, EventLoopWindowTarget},
    window::{Window, WindowId},
};
use tokio::sync::RwLock;
use wry::WebView;

use crate::{ipc_server_handler::create_ipc_name, start_event_loop_ui::UiController};

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

type BoxedCommandUI = Box<dyn FnOnce(&EventLoopWindowTarget<CustomEvent>, &mut UiController) + Send + 'static>;
pub enum CustomEvent {
    Execute(BoxedCommand),
    ExecuteUI(BoxedCommandUI),
    Exit(),
}

static ID_GENERATOR: AtomicU32 = AtomicU32::new(10);
fn get_id_generator() -> u32 {
    ID_GENERATOR.fetch_add(1, Ordering::Relaxed)
}

//#[derive(Clone)]
pub struct AppMyContext {
    hash_map: RwLock<MyWindowMap>,
    pub even_loop_poxy: EventLoopProxy<CustomEvent>,
    pub ipc_name: String,
    pub is_exit: Arc<AtomicBool>,
}

impl AppMyContext {
    pub fn new(event_loop: EventLoopProxy<CustomEvent>) -> AppMyContextArc {
        Arc::new(Self {
            hash_map: RwLock::new(HashMap::new()),
            even_loop_poxy: event_loop,
            ipc_name: create_ipc_name(),
            is_exit: Arc::new(AtomicBool::new(false)),
        })
    }
    pub fn webview_add(&self, webview: WebView, window: Window) -> u32 {
        let Ok(mut hash_map) = self.hash_map.try_write() else {
            return 0;
        };

        let window_id = window.id();
        let myunsafe = UnsafeWindowMap::new(window, webview);
        let wind_32id = myunsafe._window_id;
        hash_map.insert(window_id, myunsafe);

        wind_32id
    }

    fn maket_to_exit(&self) {
        self.is_exit.store(true, Ordering::Relaxed);
    }

    pub fn webview_remove(&self, windowid: WindowId) -> bool {
        let Ok(mut hash_map) = self.hash_map.try_write() else {
            return false;
        };

        hash_map.remove(&windowid);
        let isempty = hash_map.is_empty();
        if  isempty {
            self.maket_to_exit();
        }

        isempty
    }

    // pub fn webview_find_byid(&self, wiid: u32) ->Result<Arc::<UnsafeWindowMap>,&str>{
    //     let Ok(hashnap) = self.hash_map.try_read() else {
    //         return Err("hashmap is locked");
    //     };

    //     for (key, item) in hashnap.iter() {
    //         if  item._window_id == wiid {
    //             return  Ok(Arc::new(item.clone()));
    //         }
    //     }



    //     Err("")
    // }

    pub fn webview_close(&self, wiid: u32) {
        let windowid: Option<WindowId> = match self.hash_map.try_read() {
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

    pub fn exit_window(&self) {
        let Ok(mut hashmap) = self.hash_map.try_write() else {
            return;
        };

        println!("all windows exit");

        hashmap.clear();
        _ = self.even_loop_poxy.send_event(CustomEvent::Exit());
    }
}

pub type AppMyContextArc = Arc<AppMyContext>;
