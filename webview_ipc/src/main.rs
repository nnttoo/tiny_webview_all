#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{  time::Duration};

use tokio::time::sleep;

use crate::{
    exec_comand::exec_command,
    ipc_server::{CmdMessage, IpcRoute},
    start_event_loop::create_event_loop,
};

mod app_ctx;
mod exec_comand;
mod ipc_send;
mod ipc_server;
mod ipc_server_handler;
mod open_web;
mod start_event_loop;

#[tokio::main]
async fn main() {
    println!("Hello Async!");

    let (app_ctx, mytread) = create_event_loop();

    let ipcroute = IpcRoute::new()
        .add_route("openweb", {
            let ctx_open = app_ctx.clone();
            move |msg| {
                let ctx = ctx_open.clone();
                async move {
                    let winid = match open_web::open_web(ctx, msg.message) {
                        Ok(w) => w,
                        _ => 0,
                    };

                    CmdMessage {
                        cmd: "ini".into(),
                        message: format!("{}", winid),
                    }
                }
            }
        })
        .add_route("closeweb", {
            let ctx_open = app_ctx.clone();
            move |msg| {
                let ctx = ctx_open.clone();
                async move {
                    if let Ok(wid) = msg.message.parse::<u32>() {
                        ctx.webview_close(wid);
                    }

                    CmdMessage {
                        cmd: "".into(),
                        message: "".into(),
                    }
                }
            }
        });

    ipcroute.create_server(app_ctx.clone()); 
    tokio::spawn(exec_command(app_ctx));
    _ = mytread.await;
}
