use std::{ 
    time::{SystemTime, UNIX_EPOCH},
};

use interprocess::local_socket::{
    GenericNamespaced, // 2. Ubah ke GenericNamespacedName
    ListenerOptions,
    ToNsName, // 1. Ubah ke ToNsName
    tokio::Stream,
    traits::tokio::Listener as _,
};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{app_ctx::AppMyContextArc, open_web::open_web};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CmdMessage {
    pub cmd: String,
    pub message: String,
}

pub fn create_ipc_name() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    // Ubah ke bentuk Hexadecimal (String)
    let random_string = format!("myipc-{:x}", nanos);

    random_string
}

async fn handle_client(mut stream: Stream, app_ctx: AppMyContextArc) {
    let mut buffer = vec![0; 1024 * 1024];
    let Ok(n) = stream.read(&mut buffer).await else {
        return;
    };

    let Ok(data_from_client) = rmp_serde::from_slice::<CmdMessage>(&buffer[..n]) else {
        return;
    };
    let response = handle_cmd(data_from_client, app_ctx).await;
    _ = stream.write_all(&response).await;
    _ = stream.flush().await;
}

async fn handle_cmd(client_msg: CmdMessage, app_ctx: AppMyContextArc) -> Vec<u8> {
    let cresponse = match client_msg.cmd.as_str() {
        "openweb" => {
            let configstr = client_msg.message;
            let windowid = match open_web(app_ctx.clone(), configstr) {
                Ok(wid) => wid,
                _ => 0,
            };

            CmdMessage {
                cmd: "".into(),
                message: format!("{}", windowid),
            }
        }
        "closeweb" => {
            if let Ok(wid) = client_msg.message.parse::<u32>() {
                app_ctx.webview_close(wid);
            }

            CmdMessage {
                cmd: "".into(),
                message: "".into(),
            }
        }
        _ => CmdMessage {
            cmd: "".into(),
            message: "Command is empty".into(),
        },
    };

    match rmp_serde::to_vec_named(&cresponse) {
        Ok(data) => data,
        _ => b"".to_vec(),
    }
}

pub async fn create_ipc_server(app_ctx: AppMyContextArc) {
    let ipcname = &app_ctx.ipc_name;
    let Ok(server_name) = ipcname.to_string().to_ns_name::<GenericNamespaced>() else {
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
            handle_client(stream, app_clone).await;
        });
    }
}
