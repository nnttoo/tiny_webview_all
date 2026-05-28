use std::sync::mpsc;

use interprocess::local_socket::{
    GenericNamespaced, // 2. Ubah ke GenericNamespacedName
    ListenerOptions,
    ToNsName, // 1. Ubah ke ToNsName
    tokio::Stream,
    traits::tokio::Listener as _,
};
use serde::{Deserialize, Serialize};
use tao::{event_loop::EventLoopWindowTarget, window::WindowBuilder};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use wry::{PermissionResponse, WebViewBuilder, WebViewBuilderExtWindows};

use crate::app_ctx::{AppMyContext, CustomEvent};

pub struct ClientHandler {
    pub app_ctx: AppMyContext,
}

impl ClientHandler {
    async fn handle_client(&mut self, mut stream: Stream) {
        let mut buffer = vec![0; 1024 * 1024];
        let Ok(n) = stream.read(&mut buffer).await else {
            return;
        };

        let Ok(data_from_client) = rmp_serde::from_slice::<ClientMessage>(&buffer[..n]) else {
            return;
        };
        let response = self.handle_cmd(data_from_client).await;
        _ = stream.write_all(&response).await;
        _ = stream.flush().await;
    }

    async fn open_web(&mut self) -> u32 {
        let mut app_clone = self.app_ctx.clone();
        let (tx, rx) = mpsc::channel::<u32>();

        let command = Box::new(move |elwt: &EventLoopWindowTarget<CustomEvent>| {
            let window = WindowBuilder::new()
                .with_title("Splash Screen")
                .with_inner_size(tao::dpi::PhysicalSize::new(800, 500))
                .with_decorations(true) // Frameless
                .with_resizable(true)
                .build(&elwt)
                .unwrap();

            let webview = WebViewBuilder::new()
                .with_devtools(true)
                .with_autoplay(true)
                .with_https_scheme(true)
                .with_permission_handler(|kind| {
                    println!("Otomatis mengizinkan: {:?}", kind);
                    PermissionResponse::Allow
                })
                .with_url("https://google.com");

            let mywebview = webview.build(&window);
            let Ok(mywebview_un) = mywebview else {
                return;
            };

            let winid = app_clone.webview_add(mywebview_un, window);
            _ = tx.send(winid);
        });

        _ = self
            .app_ctx
            .even_loop_poxy
            .send_event(CustomEvent::Execute(command));

        let Ok(windid) = rx.recv() else {
            return 0;
        };

        windid
    }

    async fn handle_cmd(&mut self, client_msg: ClientMessage) -> Vec<u8> {
        let cresponse = match client_msg.cmd.as_str() {
            "openweb" => {
                let windowid = self.open_web().await;
                ClientMessage {
                    cmd: "".into(),
                    message: format!("{}", windowid),
                }
            }
            "closeweb" => {
                if let Ok(wid) = client_msg.message.parse::<u32>(){
                    self.app_ctx.webview_close(wid);
                }
                

                ClientMessage {
                    cmd: "ini test".into(),
                    message: "ini msg dari rust test dulu".into(),
                }
            }
            _ => ClientMessage {
                cmd: "ini test".into(),
                message: "ini msg dari rust test dulu".into(),
            },
        };

        match rmp_serde::to_vec_named(&cresponse) {
            Ok(data) => data,
            _ => b"".to_vec(),
        }
    }
}

pub async fn create_ipc_server(app_ctx: AppMyContext) {
    let Ok(server_name) = "my-ipc".to_ns_name::<GenericNamespaced>() else {
        return;
    };

    let Ok(listener) = ListenerOptions::new().name(server_name).create_tokio() else {
        return;
    };

    println!("Server IPC berjalan lewat library interprocess 2.x...");

    loop {
        let Ok(stream) = listener.accept().await else {
            continue;
        };

        let app_clone = app_ctx.clone();
        tokio::spawn(async move {
            let mut client_handler = ClientHandler { app_ctx: app_clone };
            client_handler.handle_client(stream).await;
        });
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ClientMessage {
    pub cmd: String,
    pub message: String,
}
