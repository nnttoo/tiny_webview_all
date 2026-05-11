import koffi from 'koffi';// 1. Definisikan Interface untuk mencocokkan struktur C/Rust
 
const lib = koffi.load("../rust_webview_node/target/release/webview_node.dll");


// Definisikan signature fungsi
// Di TS, kita bisa mendefinisikan ini sebagai fungsi yang mengembalikan void
const open_webview_test = lib.func('void open_webview_test()');

console.log('🚀 Membuka WebView (TS Version)...');

/**
 * PENTING:
 * Karena open_webview_test di Rust memanggil event_loop.run(),
 * pemanggilan ini bersifat BLOCKING. Node.js akan berhenti di baris ini
 * sampai jendela browser ditutup.
 */
try {
    open_webview_test();
    console.log('✅ Window ditutup.');
} catch (error) {
    console.error('❌ Terjadi error:', error);
} 
