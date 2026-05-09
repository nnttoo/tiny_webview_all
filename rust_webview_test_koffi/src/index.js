const koffi = require("koffi");

function createCBuffer(str) {
  return Buffer.from(str + "\0", "utf8");
}

const lib = koffi.load("../rust_webview_node/target/release/webview_node.dll");

// struct
const WebArg = koffi.struct("WebArg", {
  url: "char *",
  wclassname: "char *",
  title: "char *",
  custom_protocol: "char *",
  on_custom_protocol: "void *",
  width: "int",
  height: "int",
  is_kiosk: "bool",
  is_maximize: "bool",
  is_debug: "bool",
});

// functions
const openWebView = lib.func("openWebView", "void", ["WebArg *"]);   

const MyCallback = koffi.proto("void MyCallback(char *data)");

const callbackPtr = koffi.register((data) => {
    console.log("Diterima dari Webview:", data,"\n");
}, koffi.pointer(MyCallback));
 

const dataAwal = {
    url: createCBuffer("https://google.com"),
    wclassname: createCBuffer("iniclassnamenyadeh"),
    title: createCBuffer("ini judul nyo"),
    custom_protocol: createCBuffer(""),
    on_custom_protocol: callbackPtr,
    width: 600,
    height: 600,
    is_kiosk: false,
    is_maximize: false,
    is_debug: false
};
 

// Jaga agar tidak dihapus JS engine
global.__keepAlive = callbackPtr;

// 2. Alokasikan memori
const arg = koffi.alloc(WebArg, 1);

// 3. FORCE WRITE: Masukkan data objek ke dalam memori pointer 'arg'
koffi.encode(arg, WebArg, dataAwal);

console.log("Data di memori (JS):", koffi.decode(arg, WebArg));
// 3. Panggil fungsi Rust (arg sudah merupakan pointer)
openWebView(arg);

console.log("url:", arg.url.toString());
console.log("tambah:", tambah(2, 3));