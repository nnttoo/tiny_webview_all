<?php

function createCBuffer($string) {
    $len = strlen($string); 
    $buffer = FFI::new("char[" . ($len + 1) . "]", true); 
    FFI::memcpy($buffer, $string, $len); 
    return  $buffer;
}

$ffi = FFI::cdef("
   typedef struct {
        const char* url;
        void (*mycb)(void);
    } WebArg;

    void hello_world(WebArg* webconfig);
    int tambah(int a, int b);
", 
"../rust_webview_node/target/release/webview_node.dll"); // Sesuaikan path ke file DLL kamu

/// 2. Buat Callback di PHP
// Ini adalah fungsi PHP yang akan dipanggil oleh Rust
$callback = function() {
    echo "Callback dari PHP berhasil dijalankan!\n";
}; 
$arg =  $ffi->new("WebArg"); 
 
$urlbuffer = createCBuffer("https://google.com");
$arg->url = FFI::cast("char*", $urlbuffer);
$arg->mycb = $callback;  

$ffi->hello_world(FFI::addr($arg));  
echo "\n\nDiprintDIPHP : " . $arg->url . "\n\n";
?>