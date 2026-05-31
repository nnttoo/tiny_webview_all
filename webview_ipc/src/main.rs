#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{
    exec_comand::exec_command,
    ipc_server::IpcRoute,
    main_routefun::{ set_appctx_static, web_close, web_open},
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
    println!("Hello Async!");

    let (app_ctx, mytread) = create_event_loop();
    set_appctx_static(app_ctx.clone()); 

    let ipcroute = IpcRoute::new()
        .add_route("openweb", web_open)
        .add_route("closeweb", web_close);

    ipcroute.create_server(app_ctx.clone());
    tokio::spawn(exec_command(app_ctx));
    _ = mytread.await;
}
