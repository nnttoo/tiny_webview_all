use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Error, Result};
use wry::{
    RequestAsyncResponder,
    http::{HeaderName, Request, Response, header},
};

use crate::utils_tools::simple_file_exist;

// Haryanto 08 June 2026
/// ResponseTools is a struct to handle web responses.
/// This class will be executed in tokio::spawn inside the custom_protocol fn.
pub struct ResponseTools {
    pub req: Request<Vec<u8>>,
    pub public_path: PathBuf,
}

impl ResponseTools {
    fn create_response(&self, mybyte: &[u8]) -> Response<Vec<u8>> {
        let response = wry::http::Response::builder()
            .body(mybyte.to_vec())
            .unwrap();

        response
    }

    pub fn response_file(&self) -> Result<Response<Vec<u8>>> {
        let file_path = {
            let mut path = self.req.uri().path();
            if path.starts_with('/') {
                path = match path.strip_prefix("/") {
                    Some(u) => u,
                    _ => path,
                };
            }

            self.public_path.join(path.to_string())
        };

        let content_type = get_content_type(&file_path);

        if !simple_file_exist(&file_path) {
            println!("ini file pathnya {}", (&file_path).to_string_lossy());
            return Err(Error::msg("file not found"));
        }

        let ctn = fs::read(file_path)?;

        let rbuilder = Response::builder()
            .header(header::CONTENT_TYPE, content_type)
            .body(ctn)?;

        Ok(rbuilder)
    }

    pub async fn call_response(&self, res: RequestAsyncResponder) {
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
