use std::time::{SystemTime, UNIX_EPOCH};

use interprocess::local_socket::{
    GenericNamespaced, ListenerOptions, ToNsName, tokio::Stream, traits::tokio::Listener,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{
    app_ctx::AppMyContextArc,
    ipc_server::{CmdMessage, IpcRouteHashMapArc},
};

pub fn create_ipc_name() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    // Ubah ke bentuk Hexadecimal (String)
    let random_string = format!("myipc-{:x}", nanos);

    random_string
}

async fn handle_cmd(client_msg: CmdMessage, funmap: IpcRouteHashMapArc) -> Vec<u8> {
    let cresponse = (async || {
        let Some(cb) = funmap.get(&client_msg.cmd) else {
            return CmdMessage::new("", "No Fun found");
        };
        cb(client_msg).await
        
    })()
    .await;

    match rmp_serde::to_vec_named(&cresponse) {
        Ok(data) => data,
        _ => b"".to_vec(),
    }
}

async fn handle_client(mut stream: Stream, funarc: IpcRouteHashMapArc) {
    let mut len_buffer = [0u8; 4];
    if stream.read_exact(&mut len_buffer).await.is_err() {
        return; // Gagal membaca ukuran data
    }

    let data_len = u32::from_be_bytes(len_buffer) as usize;

    // Limit len optional
    if data_len > 10 * 1024 * 1024 {
        return;
    }

    let mut buffer = vec![0u8; data_len];
    if stream.read_exact(&mut buffer).await.is_err() {
        return; // Data terputus di tengah jalan
    }

    let Ok(data_from_client) = rmp_serde::from_slice::<CmdMessage>(&buffer) else {
        return;
    };
    let response = handle_cmd(data_from_client, funarc).await;

    // Haryanto 30 05 2026
    // Send data len at first byte
    let response_len = response.len() as u32;
    _ = stream.write_all(&response_len.to_be_bytes()).await;

    // Send actual data
    _ = stream.write_all(&response).await;
    _ = stream.flush().await;
    _ = stream.shutdown().await;
}

pub async fn create_ipc_server(app_ctx: AppMyContextArc, mapcb: IpcRouteHashMapArc) {
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
        let mapcb_clone = mapcb.clone();
        tokio::spawn(async move {
            handle_client(stream, mapcb_clone).await;
        });
    }
}
