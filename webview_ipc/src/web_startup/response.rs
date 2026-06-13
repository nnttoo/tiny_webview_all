use std::{
    fs,
    ops::Add,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Error, Result};
use image::EncodableLayout;
use path_clean::PathClean;
use path_slash::{PathBufExt, PathExt};
use tokio::{io::join, join};
use wry::{
    RequestAsyncResponder,
    http::{HeaderName, Request, Response, header},
};

use crate::{utils_tools::simple_file_exist, web_startup::web::WebAppCtx};

// Haryanto 08 June 2026
/// ResponseTools is a struct to handle web responses.
/// This class will be executed in tokio::spawn inside the custom_protocol fn.
pub struct ResponseTools {
    pub req: Request<Vec<u8>>,
    pub req_path: String,
    pub web_ctx: Arc<WebAppCtx>,
    pub public_path: PathBuf,
}

impl ResponseTools {
    pub fn new(req: Request<Vec<u8>>, web_ctx: Arc<WebAppCtx>) -> Self {
        let req_path = (&req).uri().path().to_string();

        let public_path = web_ctx.config.get_public_folder(); 

        Self {
            req: req,
            web_ctx: web_ctx,
            req_path,
            public_path : public_path,
        }
    }

    fn create_response(&self, mybyte: &[u8]) -> Response<Vec<u8>> {
        let response = wry::http::Response::builder()
            .body(mybyte.to_vec())
            .unwrap();

        response
    }

    pub fn response_file(&self) -> Result<Response<Vec<u8>>> {
        let file_path: PathBuf = {
            let relative_path = ".".to_string().add(&self.req_path); 
            let filepath = self.public_path.join(relative_path);
            filepath.clean()
        };

        let content_type = get_content_type(&file_path);

        if !simple_file_exist(&file_path) {
            println!(
                "ini file pathnya {}",
                (&file_path).to_slash_lossy().into_owned()
            );
            return Err(Error::msg("file not found"));
        }

        let ctn = fs::read(file_path)?;

        let rbuilder = Response::builder()
            .header(header::CONTENT_TYPE, content_type)
            .body(ctn)?;

        Ok(rbuilder)
    }

    pub async fn call_response(&self, res: RequestAsyncResponder) {
        if self.req_path.starts_with("/uiapi/") {
            let uiapiresponse = self.ui_api().await;
            res.respond(self.create_response(uiapiresponse.as_bytes()));
            return;
        }

        match self.response_file() {
            Ok(r) => {
                res.respond(r);
            }
            Err(e) => {
                res.respond(self.create_response(e.to_string().as_bytes()));
            }
        }
    }
}

fn get_content_type(uri: &PathBuf) -> &'static str {
    // Ambil ekstensi file dari URI, jika tidak ada/gagal, default ke ""
    let extension = Path::new(uri)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    // Match ekstensi dengan Content-Type yang sesuai
    match extension {
        "html" | "htm" => "text/html; charset=utf-8",
        "css" => "text/css",
        "js" | "mjs" => "text/javascript",
        "json" => "application/json",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "txt" => "text/plain; charset=utf-8",
        _ => "application/octet-stream", // Default untuk file biner tidak dikenal
    }
}
