"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.loadLib = loadLib;
const path_1 = __importDefault(require("path"));
const koffi_1 = __importDefault(require("koffi"));
let lib = null;
function loadLib() {
    if (lib != null)
        return lib;
    let pathRust_webview_core = path_1.default.join(__dirname, "../../../rust_webview_core/");
    let pathDllFile = path_1.default.join(pathRust_webview_core, "target/release/webview_node.dll");
    lib = koffi_1.default.load(pathDllFile);
    // const dialogFile = lib.func("select_file", "void", ["char*"]);
    // const get_active_window_count = lib.func("get_active_window_count", "size_t", []);
    return lib;
}
