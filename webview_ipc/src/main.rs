#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{  time::Duration};

use tokio::time::sleep;

use crate::{start_event_loop::create_event_loop, web_startup::start_by_json };

mod app_ctx;
mod start_event_loop;
mod start_event_loop_ui; 
mod web_startup;
mod utils_tools;

#[tokio::main]
async fn main() {
    let (app_ctx, mytread) = create_event_loop();
    start_by_json(app_ctx.clone()).await;

    tokio::spawn({
        let ctx = app_ctx.clone();
        async move {
            loop {
                
                sleep(Duration::from_millis(1000)).await;
                if ctx.is_exit.load(std::sync::atomic::Ordering::Relaxed) {
                    ctx.command_is_finish();
                    break ;
                }
            }
        }
    });
 

    // set_appctx_static(app_ctx.clone());

    // let ipcroute = IpcRoute::new()
    //     .add_route("openweb", web_open)
    //     .add_route("closeweb", web_close)
    //     .add_route("move", web_move)
    //     .add_route("resize", web_resize)
    //     .add_route("minimize", web_minimize)
    //     .add_route("maximize", web_maximize)
    //     .add_route("select_file", select_file)
    //     .add_route("select_folder", select_folder);

    // ipcroute.create_server(app_ctx.clone());
    // tokio::spawn(exec_command(app_ctx.clone()));

    // tokio::spawn(async move {
    //     sleep(Duration::from_millis(10000)).await;

    //     let ctx_clone = app_ctx.clone();
    //     app_ctx.call_ui_fun(move |_,ui_controller|{
    //         if ui_controller.is_window_empty(){
    //             ctx_clone.req_stop_all("no windows openned");
    //         }
    //     });

    // });

    _ = mytread.await;
}
