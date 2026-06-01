use serde::{Deserialize, Serialize};
use wry::{WebViewBuilder, http::StatusCode};

use crate::{ipc_send, open_web::BrowserConfig};

#[derive(Clone, Serialize, Deserialize)]
pub struct WebRequest {
    pub uri: String,
    pub method: String,
    pub body: Vec<u8>,
    pub content_type: String,
    status : u16
}

pub fn webvie_ipc<'a>(
    config: &BrowserConfig,
    webview_builder: WebViewBuilder<'a>,
) -> WebViewBuilder<'a> {
    if config.ipc_server.is_empty() {
        return webview_builder;
    }

    let ipc_server = config.ipc_server.to_string();

    let webview = webview_builder.with_asynchronous_custom_protocol(
        ipc_server.clone(),
        move |_id, _request, responder| {
            let web_rq = WebRequest {
                body: _request.body().to_vec(),
                uri: _request.uri().to_string(),
                method: _request.method().to_string(),
                content_type: "".to_string(),
                status : 200,
            };

            let ipc_server = ipc_server.clone();

            tokio::spawn(async move {
                let mut body = b"<h1>No Ipc Response </h1>".to_vec();
                let mut status : StatusCode = StatusCode::OK;
                let mut content_type = "text/html".to_string();

                if let Some(node_response) =  send_webreq_toipc(&ipc_server, &web_rq).await {

                    body = node_response.body;
                    status = match StatusCode::from_u16(node_response.status) {
                        Ok(sts)=>sts,
                        _=>{
                            StatusCode::NOT_FOUND
                        }
                    };
                    content_type = node_response.content_type;
                }

                let response = wry::http::Response::builder() //: 47
                    .header("Content-Type", content_type)
                    .header("Content-Length", format!("{}", body.len()))
                    .status(status)
                    .body(body)
                    .unwrap();
                responder.respond(response);
            });
        },
    );

    webview
}

async fn send_webreq_toipc(ipcserver: &str, webreq: &WebRequest) -> Option<WebRequest> {
    let Ok(data_web) = rmp_serde::to_vec_named(webreq) else {
        println!("parse request error");
        return None;
    };

    let node_data = match ipc_send::send_to_ipc(&ipcserver, &data_web).await  {
        Ok(dta)=>dta,
        Err(err)=>{
            println!("send Ipc error {err}");
            b"".to_vec()
        }
         
    };

    let Ok(web_node_res) = rmp_serde::from_slice::<WebRequest>(&node_data) else {
        println!("parse resononse error");
        return None;
    };

    Some(web_node_res)
}
