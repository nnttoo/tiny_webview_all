#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use tokio::time::sleep;

use crate::{
    exec_comand::exec_command,
    ipc_server::IpcRoute,
    main_routefun::{select_file, select_folder, set_appctx_static, web_close, web_maximize, web_minimize, web_move, web_open, web_resize},
    start_event_loop::create_event_loop,
};

mod app_ctx;
mod exec_comand;
mod ipc_send;
mod ipc_server;
mod ipc_server_handler;
mod main_routefun;
mod open_web;
mod open_web_icon;
mod open_web_ipc;
mod start_event_loop;
mod start_event_loop_ui;
mod utils_tools;

#[tokio::main]
async fn main() {
    let (app_ctx, mytread) = create_event_loop();
    set_appctx_static(app_ctx.clone());

    let ipcroute = IpcRoute::new()
        .add_route("openweb", web_open)
        .add_route("closeweb", web_close)
        .add_route("move", web_move)
        .add_route("resize", web_resize)
        .add_route("minimize", web_minimize)
        .add_route("maximize", web_maximize)
        .add_route("select_file", select_file)
        .add_route("select_folder", select_folder);

    ipcroute.create_server(app_ctx.clone());
    tokio::spawn(exec_command(app_ctx.clone()));
 
    tokio::spawn(async move {   
        sleep(Duration::from_millis(10000)).await;

        let ctx_clone = app_ctx.clone();
        app_ctx.call_ui_fun(move |_,ui_controller|{
            if ui_controller.is_window_empty(){
                ctx_clone.req_stop_all("no windows openned");
            }
        });

    });

    _ = mytread.await;
}
