import path from "path";
import koffi from 'koffi';
import { createCBuffer, sleep } from "./util/utils";


let lib: koffi.IKoffiLib | null = null;
export function loadLib() {

    if (lib != null) return lib;



    let pathRust_webview_core = path.join(__dirname, "../../../rust_webview_core/");
    let pathDllFile = path.join(pathRust_webview_core, "target/release/webview_node.dll");


    lib = koffi.load(pathDllFile);


    // const dialogFile = lib.func("select_file", "void", ["char*"]);
    // const get_active_window_count = lib.func("get_active_window_count", "size_t", []);

    return lib;
}


