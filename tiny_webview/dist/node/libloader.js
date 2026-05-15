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
function openWebView(arg) {
    const WebArg = koffi_1.default.struct('WebArg', {
        url: 'char *',
        title: 'char *',
        custom_protocol: 'char *',
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
    const webArgPointer = koffi_1.default.alloc(WebArg, 1);
    let savedPointer = null;
    let savedPointer2 = null;
    let endKeepLive;
    let interval = setInterval(() => { }, 1000 * 1000);
    let promise = new Promise((r) => {
        endKeepLive = () => {
            koffi_1.default.unregister(savedPointer);
            koffi_1.default.unregister(savedPointer2);
            interval.close();
            r();
            console.log("window closed");
        };
    });
    koffi_1.default.encode(webArgPointer, WebArg, {
        url: arg.url,
        custom_protocol: arg.customProtocol,
        title: arg.title,
        width: arg.width,
        height: arg.height,
        is_kiosk: arg.isKisok,
        is_maximize: arg.isMaximize,
        is_debug: arg.isDebug,
        on_custom_protocol: savedPointer = koffi_1.default.register(async (reqPtr, cbPtr, dataPtr) => {
            let SendResponse = koffi_1.default.decode(cbPtr, SendResponseProto);
            let res = {
                body: Buffer.from(""),
                body_len: 0,
                content_type: 'text/html',
                status: 404
            };
            let custReq = {};
            try {
                let request = koffi_1.default.decode(reqPtr, ResourceRequest);
                let buf = koffi_1.default.decode(request.body, 'uint8_t', request.body_len);
                custReq.body = Buffer.from(buf);
                custReq.method = request.method;
                custReq.uri = request.uri;
            }
            catch (err) {
                console.log(err);
            }
            try {
                let reqResult;
                if (arg.customProtocolOnRequest) {
                    reqResult = await arg.customProtocolOnRequest(custReq);
                }
                if (reqResult == null)
                    throw "";
                res = {
                    body: reqResult.body,
                    body_len: reqResult.body.length,
                    content_type: reqResult.content_type,
                    status: reqResult.status
                };
            }
            catch (error) {
                console.log(error);
            }
            SendResponse(res, dataPtr);
        }, OnCustomProtocolPtr),
        on_window_closed: savedPointer2 = koffi_1.default.register(() => {
            endKeepLive();
        }, OnWindowClosePtr),
    });
    openWebView(webArgPointer);
    return promise;
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
