import koffi from 'koffi';// 1. Definisikan Interface untuk mencocokkan struktur C/Rust
interface WebArgData {
    url: Buffer;
    wclassname: Buffer;
    title: Buffer;
    custom_protocol: Buffer;
    on_custom_protocol: any; // Pointer fungsi
    width: number;
    height: number;
    is_kiosk: boolean;
    is_maximize: boolean;
    is_debug: boolean;
}

// Helper untuk membuat buffer string null-terminated
function createCBuffer(str: string): Buffer {
    return Buffer.from(str + "\0", "utf8");
}

// 2. Load Library
const lib = koffi.load("../rust_webview_node/target/release/webview_node.dll");
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
// --- 2. Definisikan Prototype untuk SendResponse ---
// Di Rust: pub type SendResponse = extern "C" fn(response: *const ResourceResponse, *const c_void);
const SendResponseProto = koffi.proto('void SendResponse(const ResourceResponse *response, const void *userData)');
// --- 3. Definisikan Prototype untuk Callback Utama ---
// Di Rust: on_custom_protocol: extern "C" fn(*const ResourceRequest, SendResponse, *const c_void)
// PENTING: SendResponse di sini adalah sebuah CALLBACK POINTER
const OnCustomProtocolProto = koffi.proto('void OnCustomProtocol(const ResourceRequest *req, SendResponse *cb, const void *data)')
const OnCustomProtocolPtr = koffi.pointer('OnCustomProtocolPtr', OnCustomProtocolProto);
// --- 4. Definisikan Struct WebArg ---
const WebArg = koffi.struct('WebArg', {
    url: 'char *',
    wclassname: 'char *',
    title: 'char *',
    custom_protocol: 'char *',
    // KUNCI: Gunakan koffi.pointer() di sekitar prototype
    on_custom_protocol: OnCustomProtocolPtr,
    width: 'int',
    height: 'int',
    is_kiosk: 'bool',
    is_maximize: 'bool',
    is_debug: 'bool'
});

const openWebView = lib.func("openWebView", "void", [koffi.pointer(WebArg) ]);

const get_active_window_count = lib.func("get_active_window_count", "size_t", []);

let sleep = (n: number) => {
    return new Promise((r, x) => {
        setTimeout(() => {
            r(null);
        }, n);
    })
}
 


type MyCb = (res: any, dptr: any) => void;
let onSendResponseCB: MyCb | null = null;
let onDataPTr: any;

const myHandler = async (reqPtr: any, cbPtr: any, dataPtr: any) => {
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
    //@ts-ignore
    SendResponse(res, dataPtr);

};

// Daftarkan pointer fungsinya
const handlerPtr = koffi.register(myHandler, koffi.pointer(OnCustomProtocolProto));



const dataAwal: WebArgData = {
    url: createCBuffer("myprot://localhost"),
    wclassname: createCBuffer("iniclassnamenyadeh"),
    custom_protocol: createCBuffer("myprot"),
    title: createCBuffer("ini judul nyo"),
    on_custom_protocol: handlerPtr,
    width: 900,
    height: 600,
    is_kiosk: false,
    is_maximize: false,
    is_debug: false
};

// 7. Alokasi dan Encode
const arg = koffi.alloc(WebArg, 1);
koffi.encode(arg, WebArg, dataAwal);

async function test() {
    console.log("ini dulu kan ya");
    await sleep(2000);
    console.log("ini dulu kan ya333");

}

openWebView.async(arg, () => {
    console.log("webclosed");
});
 
  

(async () => {
    while (true) {
        await sleep(1000);
        console.log("waiting");
        let window = get_active_window_count() as number;
        if(window == 0){
            console.log("all window has closed");
            break;
        }
    }
})();

