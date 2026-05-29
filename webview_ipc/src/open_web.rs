use std::sync::{Arc, mpsc};

use serde::{Deserialize, Serialize};
use tao::{
    event_loop::{ EventLoopWindowTarget},
    window::{Fullscreen, WindowBuilder},
};
use wry::{PermissionResponse, WebViewBuilder, WebViewBuilderExtWindows};

use crate::app_ctx::{AppMyContext, CustomEvent};

#[derive(Clone, Serialize, Deserialize)]
struct BrowserConfig {
    pub url: String,
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub is_frameless: bool,
    pub is_resizable: bool,
    pub is_maximize: bool,
    pub is_debug: bool,
    pub is_always_ontop: bool,
    pub is_fullscreen: bool,
    pub ipc_server :  String
}

pub fn open_web(
    app_ctx: Arc::<AppMyContext>,
    configstr: String,
) -> Result<u32, Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel::<u32>();

    let config = serde_json::from_str::<BrowserConfig>(&configstr)?;

    let app_ctx_clone = app_ctx.clone();
    let command = Box::new(move |elwt: &EventLoopWindowTarget<CustomEvent>| {
        let winid = (|| -> Result<u32, Box<dyn std::error::Error>> {
            println!("berapa wya {}", config.width);
            let mut builder = WindowBuilder::new()
                .with_title(config.title)
                .with_inner_size(tao::dpi::PhysicalSize::new(config.width, config.height))
                .with_decorations(!config.is_frameless) // Frameless
                .with_resizable(config.is_resizable)
                .with_always_on_top(config.is_always_ontop)
                .with_maximized(config.is_maximize);

            if config.is_fullscreen {
                let primary_monitor = elwt.primary_monitor();
                builder = builder.with_fullscreen(Some(Fullscreen::Borderless(primary_monitor)));
            }

            let window = builder.build(&elwt)?;

            let webview = WebViewBuilder::new()
                .with_devtools(config.is_debug)
                .with_autoplay(true)
                .with_https_scheme(true)
                .with_permission_handler(|kind| {
                    println!("Otomatis mengizinkan: {:?}", kind);
                    PermissionResponse::Allow
                })
                .with_url(config.url);

            let mywebview = webview.build(&window)?;
            let winid = app_ctx_clone.webview_add(mywebview, window);
            Ok(winid)
        })();

        match winid {
            Ok(winid) => {
                _ = tx.send(winid);
            }
            Err(_) => {
                _ = tx.send(0);
            }
        }
    });

    _ = app_ctx
        .even_loop_poxy
        .send_event(CustomEvent::Execute(command));

    let windid = rx.recv()?;

    Ok(windid)
}
