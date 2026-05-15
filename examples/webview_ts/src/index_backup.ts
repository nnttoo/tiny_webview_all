import koffi from 'koffi';// 1. Definisikan Interface untuk mencocokkan struktur C/Rust
interface WebArgData {
    url: Buffer;
    wclassname: Buffer;
    title: Buffer;
    custom_protocol: Buffer;
    on_custom_protocol: any; // Pointer fungsi
    on_window_closed: any;
    width: number;
    height: number;
    is_kiosk: boolean;
    is_maximize: boolean;
    is_debug: boolean;
}

function createCBuffer(str: string): Buffer {
    return Buffer.from(str + "\0", "utf8");
}

// 2. Load Library
const lib = koffi.load("../../rust_webview_core/target/release/webview_node.dll");
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

const dialogFile = lib.func("select_file", "void", ["char*"]);
const openWebView = lib.func("openWebView", "void", [koffi.pointer(WebArg)]);
const get_active_window_count = lib.func("get_active_window_count", "size_t", []);
let sleep = (n: number) => {
    return new Promise((r, x) => {
        setTimeout(() => {
            r(null);
        }, n);
    })
}


let savedPointer: any;
let savedPointer2: any;



// const dataAwal: WebArgData = {
//     url: createCBuffer("myprot://localhost"),
//     wclassname: createCBuffer("iniclassnamenyadeh"),
//     custom_protocol: createCBuffer("myprot"),
//     title: createCBuffer("ini judul nyo"),
//     on_custom_protocol: savedPointer = koffi.register(
//         async (reqPtr: any, cbPtr: any, dataPtr: any) => {
//             console.log("🔥 BOOM! Callback terpanggil!");
//             console.log("kita coba buat ia menungu");
//             console.log(" menungu selesai");
//             let SendResponse = koffi.decode(cbPtr, SendResponseProto);

//             await sleep(2000);

//             const res = {
//                 body: Buffer.from("Halo!"),
//                 body_len: 5,
//                 content_type: "text/plain",
//                 status: 200
//             };
//             //@ts-ignore
//             SendResponse(res, dataPtr);

//         },
//         OnCustomProtocolPtr
//     ),
//     on_window_closed : savedPointer2 = koffi.register(
//         ()=>{
//             console.log("ini setelah ditutup");
//         },
//         OnWindowClosePtr
//     ),
//     width: 900,
//     height: 600,
//     is_kiosk: false,
//     is_maximize: false,
//     is_debug: false
// };

// 7. Alokasi dan Encode


const arg = koffi.alloc(WebArg, 1);
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

interface FileType {
    file_name: string,
    ext: string[]
}
interface FileSelectorArg{
    root_dir : string,
    file_types : FileType[]
}

function dialogFileJs(fileex : FileSelectorArg){
    let jsontxt = JSON.stringify(fileex);

    dialogFile(jsontxt);
}

sleep(2000).finally(() => {
    dialogFileJs({
        root_dir : "C:\\Users\\Anto\\Downloads\\New folder",
        file_types : [{
            ext : ["zip","exe"],
            file_name : "Sembarang"
        }]
    })
});


(async () => {
    while (true) {
        await sleep(1000);
        console.log("waiting");
        let window = get_active_window_count() as number;
        if (window == 0) {
            console.log("all window has closed");
            break;
        }
    }
})();


