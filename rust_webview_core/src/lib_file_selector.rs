use std::ffi::c_char;

use rfd::FileDialog;
use serde::Deserialize;

use crate::webconfig::{get_string_from_cpointer, string_tocstring};

#[derive(Deserialize, Debug)]
pub struct FileType {
    pub file_name: String,
    pub ext: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct FileSelectorArg {
    pub root_dir: String,
    pub file_types: Vec<FileType>,
}

#[unsafe(no_mangle)]
pub extern "C" fn select_file(json_config: *const c_char, callback: extern "C" fn(*const c_char)) {
    let jsonstr = get_string_from_cpointer(json_config);
    let nullcstring = string_tocstring(String::from(""));

    let configresult = match serde_json::from_str::<FileSelectorArg>(&jsonstr) {
        Ok(v) => v,
        Err(_) => {
            callback(nullcstring.as_ptr());
            return;
        }
    };

    let mut file_d = FileDialog::new().set_directory(configresult.root_dir);

    for item in configresult.file_types {
        file_d = file_d.add_filter(item.file_name, &item.ext);
    }

    let file = file_d.pick_file();

    match file {
        Some(path) => {
            let path_str = path.to_string_lossy().into_owned();
            callback(string_tocstring(path_str).as_ptr());
        }
        None => {
            println!("User membatalkan pilihan.");
            callback(nullcstring.as_ptr());
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn select_folder(rootfolder: *const c_char, callback: extern "C" fn(*const c_char)) {
    let nullcstring = string_tocstring(String::from(""));
    let rootfolder_c = get_string_from_cpointer(rootfolder);
    let file_d = FileDialog::new().set_directory(rootfolder_c);
    let file = file_d.pick_folder();

    match file {
        Some(path) => {
            let path_str = path.to_string_lossy().into_owned();
            callback(string_tocstring(path_str).as_ptr());
        }
        None => {
            println!("User membatalkan pilihan.");
            callback(nullcstring.as_ptr());
        }
    }
}
