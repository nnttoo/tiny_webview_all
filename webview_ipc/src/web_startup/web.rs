use std::{path::PathBuf, sync::Arc};

use tao::{
    event_loop::EventLoopWindowTarget,
    window::{Fullscreen, WindowBuilder},
};
use wry::{PermissionResponse, RequestAsyncResponder, WebViewBuilder, WebViewId, http::Request};

use crate::{
    app_ctx::AppMyContextArc, start_event_loop::CustomEvent, start_event_loop_ui::UiController, utils_tools::check_current_thread, web_startup::{icon::load_dynamic_png, response_tools::ResponseTools} 
};

#[derive(Clone)]
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
    pub ipc_public_folder: PathBuf,
}

#[derive(Clone)] 
pub struct WebAppCtx {
    pub config: Arc<BrowserConfig>,
    pub ctx: AppMyContextArc,
}

impl WebAppCtx {
    pub fn into_arc(self) -> Arc<WebAppCtx> {
        Arc::new(self)
    }

    ///
    /// open web on ui thread
    ///
    /// because this fn will call from anothre thread,
    /// we need to using Arc
    /// so we can clone the Arc easly
    ///
    pub fn open_web(self : Arc<Self>) {
        let app_ctx = &self.ctx; 
 
        

        let opener_arc =  self.clone();
        app_ctx.call_ui_fun(move |elwt, ui_controller| {
            _ = opener_arc.open_web_ui(elwt, ui_controller);
        });
    }

    pub fn test(&self){

        let isuithread = self.ctx.is_ui_thread();

        println!("halo ini test aja dulu {}",isuithread);
    }


    ///
    /// Call from Ui Thread
    /// 
    fn open_web_ui(
        &self,
        elwt: &EventLoopWindowTarget<CustomEvent>,
        ui_controller: &mut UiController 
    ) -> Result<u32, Box<dyn std::error::Error>> {

        let config = self.config.clone();

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
            builder = builder.with_fullscreen(Some(Fullscreen::Borderless(primary_monitor)));
        }

        let window = builder.build(&elwt)?;

        let webview = WebViewBuilder::new()
            .with_devtools(config.is_debug)
            .with_autoplay(true)
            .with_permission_handler(|kind| {
                println!("Otomatis mengizinkan: {:?}", kind);
                PermissionResponse::Allow
            })
            .with_asynchronous_custom_protocol("myapp".into(), self.custom_protocol())
            .with_url(config.url.to_string());

        #[cfg(target_os = "windows")]
        {
            //webview = webview.with_https_scheme(true);
        }

        //  webview = custom_protocol(webview, "myapp");

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
    }
 

    ///
    /// Listener for with_asynchronous_custom_protocol
    /// 
    
    fn custom_protocol(
        &self ,
    ) -> Box<dyn Fn(WebViewId, Request<Vec<u8>>, RequestAsyncResponder)> {

        let self_clone = self.clone();

        Box::new(move |_id, _request, responder| { 
            
            let public_path = self_clone.config.ipc_public_folder.clone();

            tokio::spawn(async move {  

                ResponseTools::new(_request, public_path).call_response(responder).await; 

            });
        })
    }
}
