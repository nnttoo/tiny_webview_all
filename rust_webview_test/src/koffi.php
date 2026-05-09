<?php

function createCBuffer($string) {
    $len = strlen($string); 
    $buffer = FFI::new("char[" . ($len + 1) . "]", false); 
    FFI::memcpy($buffer, $string, $len); 
    return   $buffer;
}

$ffi = FFI::cdef("
   typedef struct {
        char* url;
        char* wclassname;
        char* title;
        char* custom_protocol;

        void (*on_custom_protocol)(const char*);

        int width;
        int height;

        bool is_kiosk;
        bool is_maximize;
        bool is_debug;

    } WebArg;

    void openWebView(WebArg* webconfig); 
", 
"../rust_webview_node/target/release/webview_node.dll"); // Sesuaikan path ke file DLL kamu

/// 2. Buat Callback di PHP
// Ini adalah fungsi PHP yang akan dipanggil oleh Rust
$callback = function($data) { 
    echo "ini contoh data : `" . $data;
    echo "Callback dari PHP berhasil dijalankan!\n";
}; 
$arg =  $ffi->new("WebArg"); 
  
$arg->url =  createCBuffer("https://google.com");
$arg->wclassname = createCBuffer("iniclassnamenyadeh");
$arg->width = 600;
$arg->height = 600;
$arg->is_maximize = false;
$arg->title =  createCBuffer("ini judul nyo");
$arg->is_kiosk = false;
$arg->on_custom_protocol = $callback;  

$ffi->openWebView(FFI::addr($arg));  
echo "\n\nDiprintDIPHP : " . FFI::string($arg->url) . "\n\n";
?>