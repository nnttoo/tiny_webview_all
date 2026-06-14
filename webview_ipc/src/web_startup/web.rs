use std::{ path::PathBuf, sync::Arc};

use tao::{
    event_loop::EventLoopWindowTarget,
    window::{Fullscreen, WindowBuilder},
};
use wry::{PermissionResponse, RequestAsyncResponder, WebViewBuilder, WebViewId, http::Request};

use crate::{
    app_ctx::AppMyContextArc, start_event_loop::CustomEvent, start_event_loop_ui::UiController,  web_startup::{icon::load_dynamic_png, response::ResponseTools, response_command::CommandManager, startup::WindowJson} 
};

#[derive(Clone)]
pub struct BrowserConfig {
    pub win_json : Arc<WindowJson>,
    pub uri :  String,
    pub current_folder : PathBuf,
    pub custom_protocol : String,
}

impl  BrowserConfig {
    pub fn get_public_folder(&self)->PathBuf{
        let parent = match self.current_folder.parent(){
            Some(p)=>p,
            _=>&self.current_folder
        };

        let current_path = parent.join(&self.win_json.public_folder); 
        current_path
    }
}


 
pub struct WebAppCtx {
    pub config: Arc<BrowserConfig>,
    pub ctx: AppMyContextArc,
    pub command_mager : Arc<CommandManager>
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

    ///
    /// Call from Ui Thread
    /// 
    fn open_web_ui(
        self : Arc<Self>,
        elwt: &EventLoopWindowTarget<CustomEvent>,
        ui_controller: &mut UiController 
    ) -> Result<u32, Box<dyn std::error::Error>> {

        let wjson = &self.config.win_json;
        let url = &self.config.uri;  
        let custom_protocol = &self.config.custom_protocol;

        let mut builder = WindowBuilder::new()
            .with_title(wjson.title.to_string())
            .with_inner_size(tao::dpi::PhysicalSize::new(wjson.width, wjson.height))
            .with_decorations(!wjson.is_frameless) // Frameless
            .with_resizable(wjson.is_resizable)
            .with_window_icon(load_dynamic_png())
            .with_always_on_top(wjson.is_always_ontop)
            .with_maximized(wjson.is_maximize);

        if wjson.is_fullscreen {
            let primary_monitor = elwt.primary_monitor();
            builder = builder.with_fullscreen(Some(Fullscreen::Borderless(primary_monitor)));
        }

        let window = builder.build(&elwt)?;

        let webview = WebViewBuilder::new()
            .with_devtools(wjson.is_debug)
            .with_autoplay(true)
            .with_permission_handler(|kind| {
                println!("Otomatis mengizinkan: {:?}", kind);
                PermissionResponse::Allow
            })
            .with_asynchronous_custom_protocol(custom_protocol.to_string(), self.clone().custom_protocol())
            .with_url(url.to_string());

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
        self :Arc<Self>,
    ) -> Box<dyn Fn(WebViewId, Request<Vec<u8>>, RequestAsyncResponder)> { 
        let self_clone =  self.clone();

        Box::new(move |_id, _request, responder| { 
            let self_clone  = self_clone.clone();
            tokio::spawn(async move {  

                ResponseTools::new(
                    _request, 
                    self_clone
                
                ).call_response(responder).await; 

            });
        })
    }
}
