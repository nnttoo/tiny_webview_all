<?php
 
$ffi = FFI::cdef("
    void hello_world();
    int tambah(int a, int b);
", "../rust_webview_node/target/release/webview_node.dll"); // Sesuaikan path ke file DLL kamu

// 2. Panggil fungsi hello_world
// Ini akan muncul di terminal/output server kamu (stdout)
$ffi->hello_world();

// 3. Panggil fungsi tambah
$hasil = $ffi->tambah(10, 20);

echo "Hasil dari Rust: " . $hasil; // Output: Hasil dari Rust: 30
?>