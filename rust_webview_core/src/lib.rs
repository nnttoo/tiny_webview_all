mod selector_arg;
mod webconfig;
mod webview_open;
mod lib_webview;

// #[unsafe(no_mangle)]
// pub extern "C" fn get_active_window_count() -> usize {
//     if let Ok(map) = WEBVIEWS.get_or_init(|| Mutex::new(HashMap::new())).lock() {
//         let count = map.len();
//         count
//     } else {
//         0 // Jika gagal lock, anggap 0 (aman)
//     }
// }

// #[unsafe(no_mangle)]
// pub extern "C" fn select_file(json_config: *const c_char) {
//     let jsonstr = get_string_from_cpointer(json_config);
//     let configresult = match serde_json::from_str::<FileSelectorArg>(&jsonstr) {
//         Ok(v) => v,
//         Err(_) => return,
//     };

//     let mut file_d = FileDialog::new().set_directory(configresult.root_dir);

//     for item in configresult.file_types {
//         file_d = file_d.add_filter(item.file_name, &item.ext);
//     }

//     let file = file_d.pick_file();

//     match file {
//         Some(path) => println!("User memilih: {:?}", path),
//         None => println!("User membatalkan pilihan."),
//     }
// }
