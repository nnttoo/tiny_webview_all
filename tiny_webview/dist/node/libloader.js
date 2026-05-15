"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.openWebView = openWebView;
exports.keepLive = keepLive;
const path_1 = __importDefault(require("path"));
const koffi_1 = __importDefault(require("koffi"));
const utils_1 = require("./util/utils");
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
function openWebView() {
    const ResourceRequest = koffi_1.default.struct('ResourceRequest', {
        uri: 'char *',
        method: 'char *',
        body: 'uint8_t *',
        body_len: 'size_t',
    });
    const ResourceResponse = koffi_1.default.struct('ResourceResponse', {
        body: 'uint8_t *',
        body_len: 'size_t',
        content_type: 'char *',
        status: 'int',
    });
    const SendResponseProto = koffi_1.default.proto('void SendResponse(const ResourceResponse *response, const void *userData)');
    const OnCustomProtocolPtr = koffi_1.default.pointer('OnCustomProtocolPtr', koffi_1.default.proto('void OnCustomProtocol(const ResourceRequest *req, SendResponse *cb, const void *data)'));
    const OnWindowClosePtr = koffi_1.default.pointer("OnWindowClosePtr", koffi_1.default.proto("void OnWindowClose()"));
    const WebArg = koffi_1.default.struct('WebArg', {
        url: 'char *',
        wclassname: 'char *',
        title: 'char *',
        custom_protocol: 'char *',
        // KUNCI: Gunakan koffi.pointer() di sekitar prototype
        on_custom_protocol: OnCustomProtocolPtr,
        on_window_closed: OnWindowClosePtr,
        width: 'int',
        height: 'int',
        is_kiosk: 'bool',
        is_maximize: 'bool',
        is_debug: 'bool'
    });
    let lib = loadLib();
    const openWebView = lib.func("openWebView", "void", [koffi_1.default.pointer(WebArg)]);
    const arg = koffi_1.default.alloc(WebArg, 1);
    let savedPointer = null;
    let savedPointer2 = null;
    koffi_1.default.encode(arg, WebArg, {
        url: (0, utils_1.createCBuffer)("myprot://localhost"),
        wclassname: (0, utils_1.createCBuffer)("iniclassnamenyadeh"),
        custom_protocol: (0, utils_1.createCBuffer)("myprot"),
        title: (0, utils_1.createCBuffer)("ini judul nyo"),
        on_custom_protocol: savedPointer = koffi_1.default.register(async (reqPtr, cbPtr, dataPtr) => {
            console.log("🔥 BOOM! Callback terpanggil!");
            console.log("kita coba buat ia menungu");
            console.log(" menungu selesai");
            let SendResponse = koffi_1.default.decode(cbPtr, SendResponseProto);
            await (0, utils_1.sleep)(2000);
            const res = {
                body: Buffer.from("Halo!"),
                body_len: 5,
                content_type: "text/plain",
                status: 200
            };
            SendResponse(res, dataPtr);
        }, OnCustomProtocolPtr),
        on_window_closed: savedPointer2 = koffi_1.default.register(() => {
            console.log("ini setelah ditutup");
        }, OnWindowClosePtr),
        width: 900,
        height: 600,
        is_kiosk: false,
        is_maximize: false,
        is_debug: false
    });
    openWebView(arg);
}
async function keepLive() {
    let lib = loadLib();
    const get_active_window_count = lib.func("get_active_window_count", "size_t", []);
    while (true) {
        await (0, utils_1.sleep)(1000);
        let window = get_active_window_count();
        if (window == 0) {
            console.log("all window has closed");
            break;
        }
    }
}
