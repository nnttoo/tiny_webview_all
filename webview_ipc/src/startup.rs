use std::{fs::exists, path::PathBuf, sync::Arc};

use native_dialog::{MessageDialog, MessageType};
use rmp_serde::encode::write;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{
    app_ctx::AppMyContextArc,
    startup_web::{BrowserConfig, BrowserOpener},
    utils_tools::get_exe_folder,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct WindowJson {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub is_frameless: bool,
    pub is_resizable: bool,
    pub is_maximize: bool,
    pub is_debug: bool,
    pub is_always_ontop: bool,
    pub is_fullscreen: bool,

    pub public_folder: String,
}

impl WindowJson {
    pub fn new() -> Self {
        Self {
            width: 900,
            height: 700,
            title: "Webview Ipc".into(),
            is_always_ontop: false,
            is_debug: true,
            is_frameless: false,
            is_fullscreen: false,
            is_maximize: false,
            is_resizable: true,

            public_folder: "./public".into(),
        }
    }
}

fn file_exist(path: &PathBuf) -> bool {
    let Ok(exist) = exists(path) else {
        return false;
    };

    return exist;
}

pub async fn start_by_json(appctx: AppMyContextArc) {
    let filename = "window.json";
    let mut filepath = get_exe_folder().join(filename);

    if !file_exist(&filepath) {
        filepath = filename.into();
    }

    let mut window_json = WindowJson::new();

    if !file_exist(&filepath) {
        let str = match serde_json::to_string_pretty(&window_json) {
            Ok(str) => str,
            _ => "".into(),
        };

        _ = MessageDialog::new()
            .set_type(MessageType::Warning)
            .set_title("No window.json found")
            .set_text(&str)
            .show_alert();

        _ = fs::write(&filepath, str).await;
        appctx.command_is_finish();
        return;
    } else {
        match fs::read_to_string(&filepath).await {
            Ok(filectn) => match serde_json::from_str(&filectn) {
                Ok(win_json) => {
                    window_json = win_json;
                }

                _ => {}
            },
            _ => {}
        }
    }

    let browser_opener = BrowserOpener {
        config: BrowserConfig {
            height: window_json.height,
            title: window_json.title,
            ipc_server: "".into(),
            is_always_ontop: window_json.is_always_ontop,
            is_debug: window_json.is_debug,
            is_frameless: window_json.is_frameless,
            is_fullscreen: window_json.is_fullscreen,
            is_maximize: window_json.is_maximize,
            is_resizable: window_json.is_resizable,
            url: "myapp://localhost/index.html".into(),
            width: window_json.width,
            ipc_public_folder: filepath.join(window_json.public_folder),
        },
        ctx: appctx,
    }.into_arc(); 

    browser_opener.open_web();
}
