use std::sync::{Arc, mpsc};

use crate::{
    app_ctx::AppMyContext, open_web_icon::load_dynamic_png, open_web_ipc::webvie_ipc,
    start_event_loop::CustomEvent, start_event_loop_ui::UiController,
};
use native_dialog::{MessageDialog, MessageType};
use serde::{Deserialize, Serialize};
use tao::{
    event_loop::EventLoopWindowTarget,
    window::{Fullscreen, WindowBuilder},
};
#[cfg(target_os = "windows")]
use wry::WebViewBuilderExtWindows as _;
use wry::{PermissionResponse, WebViewBuilder};

#[derive(Clone, Serialize, Deserialize)]
pub struct BrowserConfig {
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
    pub ipc_server: String,
}

pub fn open_web(
    app_ctx: Arc<AppMyContext>,
    configstr: String,
) -> Result<u32, Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel::<u32>();

    let config = serde_json::from_str::<BrowserConfig>(&configstr)?;

    let command = Box::new(
        move |elwt: &EventLoopWindowTarget<CustomEvent>, ui_controller: &mut UiController| {
            let winid = (|| -> Result<u32, Box<dyn std::error::Error>> {
                let mut builder = WindowBuilder::new()
                    .with_title(config.title.to_string())
                    .with_inner_size(tao::dpi::PhysicalSize::new(config.width, config.height))
                    .with_decorations(!config.is_frameless) // Frameless
                    .with_resizable(config.is_resizable)
                    .with_window_icon(load_dynamic_png())
                    .with_always_on_top(config.is_always_ontop)
                    .with_maximized(config.is_maximize);

                if config.is_fullscreen {
                    let primary_monitor = elwt.primary_monitor();
                    builder =
                        builder.with_fullscreen(Some(Fullscreen::Borderless(primary_monitor)));
                }

                let window = builder.build(&elwt)?;

                let mut webview = WebViewBuilder::new()
                    .with_devtools(config.is_debug)
                    .with_autoplay(true)
                    .with_permission_handler(|kind| {
                        println!("Otomatis mengizinkan: {:?}", kind);
                        PermissionResponse::Allow
                    })
                    .with_url(config.url.to_string());

                #[cfg(target_os = "windows")]
                {
                    webview = webview.with_https_scheme(true);
                }

                webview = webvie_ipc(&config, webview);

                #[cfg(target_os = "windows")]
                {
                    let mut data_dir = std::env::var("LOCALAPPDATA")
                        .map(std::path::PathBuf::from)
                        .unwrap_or_else(|_| std::env::current_dir().unwrap());
                    data_dir.push("TinyWebView");
                    data_dir.push("WebViewData");
                    let _ = std::fs::create_dir_all(&data_dir);
                    unsafe {
                        std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", data_dir.to_str().unwrap());
                    }
                }

                let mywebview = webview.build(&window)?;

                let winid = ui_controller.add(window, mywebview);
                Ok(winid)
            })();

            match winid {
                Ok(winid) => {
                    _ = tx.send(winid);
                }
                Err(e) => {
                    _ = tx.send(0);

                    println!("Failed to initialize WebView runtime: {:?}", e);

                    // Ask the user if they want to download the runtime
                    let download_confirmed = MessageDialog::new()
                    .set_type(MessageType::Warning)
                    .set_title("WebView2 Runtime Required")
                    .set_text("The required WebView2 runtime was not found.\n\nWould you like to open the official Microsoft page to download and install it?")
                    .show_confirm() // Returns true if user clicks Yes/OK
                    .unwrap_or(false);

                    if download_confirmed {
                        let download_url = "https://developer.microsoft.com/en-us/microsoft-edge/webview2/#download-section";
                        println!("Opening download link: {}", download_url);

                        // Open the link automatically in the user's default browser
                        if let Err(e) = open::that(download_url) {
                            eprintln!("Failed to open browser link: {:?}", e);
                        }
                    }
                }
            }
        },
    );

    _ = app_ctx
        .even_loop_poxy
        .send_event(CustomEvent::ExecuteUI(command));

    let windid = rx.recv()?;

    Ok(windid)
}
