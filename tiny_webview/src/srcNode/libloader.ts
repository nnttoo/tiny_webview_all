import path from "path";
import koffi from 'koffi';
import { createCBuffer, sleep } from "./util/utils";


let lib: koffi.IKoffiLib | null = null;



function loadLib() {

    if (lib != null) return lib;



    let pathRust_webview_core = path.join(__dirname, "../../../rust_webview_core/");
    let pathDllFile = path.join(pathRust_webview_core, "target/release/webview_node.dll");


    lib = koffi.load(pathDllFile);


    // const dialogFile = lib.func("select_file", "void", ["char*"]);
    // const get_active_window_count = lib.func("get_active_window_count", "size_t", []);

    return lib;
}


export async function openWebView() {
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

    const WebArg = koffi.struct('WebArg', {
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
    const openWebView = lib.func("openWebView", "void", [koffi.pointer(WebArg)]);


    const arg = koffi.alloc(WebArg, 1);

    let savedPointer: any = null;
    let savedPointer2: any = null;

    let isWindowOnClosed = false;

    koffi.encode(arg, WebArg, {
        url: createCBuffer("myprot://localhost"),
        wclassname: createCBuffer("iniclassnamenyadeh"),
        custom_protocol: createCBuffer("myprot"),
        title: createCBuffer("ini judul nyo"),
        on_custom_protocol: savedPointer = koffi.register(
            async (reqPtr: any, cbPtr: any, dataPtr: any) => {
                console.log("🔥 BOOM! Callback terpanggil!");
                console.log("kita coba buat ia menungu");
                console.log(" menungu selesai");
                let SendResponse = koffi.decode(cbPtr, SendResponseProto);

                await sleep(2000);

                const res = {
                    body: Buffer.from("Halo!"),
                    body_len: 5,
                    content_type: "text/plain",
                    status: 200
                };
                SendResponse(res, dataPtr);

            },
            OnCustomProtocolPtr
        ),
        on_window_closed: savedPointer2 = koffi.register(
            () => {
                console.log("ini setelah ditutup");
                isWindowOnClosed = true;
            },
            OnWindowClosePtr
        ),
        width: 900,
        height: 600,
        is_kiosk: false,
        is_maximize: false,
        is_debug: false
    });

    openWebView(arg);

    while(true){
        await sleep(2000);
        if(isWindowOnClosed) break;
    }

    koffi.unregister(savedPointer);
    koffi.unregister(savedPointer2);
    console.log("window closed");

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


