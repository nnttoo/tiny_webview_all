 
use crate::start_event_loop::{ create_event_loop};

mod ipc_server;
mod start_event_loop;
mod app_ctx; 

#[tokio::main]
async fn main() {
    println!("Hello Async!");


    let (app_ctx, mytread) = create_event_loop();  
    tokio::spawn(ipc_server::create_ipc_server(app_ctx));


    // let command = Box::new(move |elwt: &EventLoopWindowTarget<CustomEvent>, appctx : AppMyContext| {

    //     let Ok(mut locked) = appctx.hash_map.try_lock() else {
    //         return;
    //     };

    //     let window = WindowBuilder::new()
    //         .with_title("Splash Screen")
    //         .with_inner_size(tao::dpi::PhysicalSize::new(800, 500))
    //         .with_decorations(true) // Frameless
    //         .with_resizable(true)
    //         .build(&elwt)
    //         .unwrap();
 

    //     let webview = WebViewBuilder::new()
    //         .with_devtools(true)
    //         .with_autoplay(true)
    //         .with_https_scheme(true)
    //         .with_permission_handler(|kind| {
    //             println!("Otomatis mengizinkan: {:?}", kind);
    //             PermissionResponse::Allow
    //         })
    //         .with_url("https://google.com");

    //     let mywebview = webview.build(&window);  
    //     locked.insert(window.id(), UnsafeWindowMap{
    //         _webview : mywebview.unwrap(),
    //         _window : window
    //     });


    // });

    // _ = app_ctx.even_loop_poxy.send_event(CustomEvent::Execute(command));
    _ = mytread.await;
}
