import path from "path";
import koffi from 'koffi';
import { createCBuffer, sleep } from "./util/utils";


let lib: koffi.IKoffiLib | null = null;
const ResourceRequest = koffi.struct('ResourceRequest', {
    uri: 'char *',
    method: 'char *',
    body: 'uint8_t *',
    body_len: 'size_t',
});
const ResourceResponse = koffi.struct('ResourceResponse', {
    body: 'uint8_t *',
    body_len: 'size_t',
    content_type: 'char *',
    status: 'int',
});
const SendResponseProto = koffi.proto('void SendResponse(const ResourceResponse *response, const void *userData)');
const OnCustomProtocolPtr = koffi.pointer(
    'OnCustomProtocolPtr',
    koffi.proto('void OnCustomProtocol(const ResourceRequest *req, SendResponse *cb, const void *data)')
);

const OnWindowClosePtr = koffi.pointer(
    "OnWindowClosePtr",
    koffi.proto("void OnWindowClose()")
)


function loadLib() {

    if (lib != null) return lib;



    let pathRust_webview_core = path.join(__dirname, "../../../rust_webview_core/");
    let pathDllFile = path.join(pathRust_webview_core, "target/release/webview_node.dll");


    lib = koffi.load(pathDllFile);


    // const dialogFile = lib.func("select_file", "void", ["char*"]);
    // const get_active_window_count = lib.func("get_active_window_count", "size_t", []);

    return lib;
}

export interface CustomProtocolResponse {
    body: Buffer<ArrayBuffer>,
    content_type: string,
    status: number,
}

export interface CustomProtocolRequest {
    uri: string,
    method: string,
    body: Buffer<ArrayBuffer>,
}

export interface WebConfig {
    url: string, 
    customProtocol?: string,
    customProtocolOnRequest?: (p: CustomProtocolRequest) => Promise<CustomProtocolResponse>,
    title: string,
    width: number,
    height: number,
    isKisok: boolean,
    isMaximize: boolean,
    isDebug: boolean,
}

export function openWebView(arg: WebConfig) {


    const WebArg = koffi.struct('WebArg', {
        url: 'char *',
        wclassname: 'char *',
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
    const openWebView = lib.func("openWebView", "void", [koffi.pointer(WebArg)]);
    const webArgPointer = koffi.alloc(WebArg, 1);

    let savedPointer: any = null;
    let savedPointer2: any = null;

    let endKeepLive: () => void;
    let interval = setInterval(() => { }, 1000 * 1000);
    let promise = new Promise<void>((r) => {
        endKeepLive = () => {

            koffi.unregister(savedPointer);
            koffi.unregister(savedPointer2);
            interval.close();
            r();

            console.log("window closed");
        }
    });

    koffi.encode(webArgPointer, WebArg, {
        url: arg.url,
        wclassname: "wnd" + Date.now() ,
        custom_protocol: arg.customProtocol,
        title: arg.title,

        width: arg.width,
        height: arg.height,
        is_kiosk: arg.isKisok,
        is_maximize: arg.isMaximize,
        is_debug: arg.isDebug,

        on_custom_protocol: savedPointer = koffi.register(
            async (reqPtr: any, cbPtr: any, dataPtr: any) => {
                let SendResponse = koffi.decode(cbPtr, SendResponseProto);

                let res = {
                    body: Buffer.from(""),
                    body_len: 0,
                    content_type: 'text/html',
                    status: 404
                };

                let custReq = {} as CustomProtocolRequest;

                try {

                    let request = koffi.decode(reqPtr, ResourceRequest);
                    let buf = koffi.decode(request.body, 'uint8_t', request.body_len);

                    custReq.body = Buffer.from(buf);
                    custReq.method = request.method;
                    custReq.uri = request.uri;


                } catch (err) {
                    console.log(err);
                }

                try {


                    let reqResult: any

                    if (arg.customProtocolOnRequest) {
                        reqResult = await arg.customProtocolOnRequest(custReq);
                    }

                    if(reqResult == null) throw ""


                    res = {
                        body: reqResult.body,
                        body_len: reqResult.body.length,
                        content_type: reqResult.content_type,
                        status: reqResult.status
                    };
                } catch (error) {
                    console.log(error);
                }

                SendResponse(res, dataPtr);


            },
            OnCustomProtocolPtr
        ),
        on_window_closed: savedPointer2 = koffi.register(
            () => {
                console.log("ini setelah ditutup");
                endKeepLive();
            },
            OnWindowClosePtr
        ),
    });

    openWebView(webArgPointer);
    return promise;

}

export async function keepLive() {


    let lib = loadLib();
    const get_active_window_count = lib.func("get_active_window_count", "size_t", []);

    while (true) {
        await sleep(1000);
        let window = get_active_window_count() as number;
        if (window == 0) {
            console.log("all window has closed");
            break;
        }
    }
}


