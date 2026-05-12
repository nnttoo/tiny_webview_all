<?php

function createCBuffer($string) {
    $len = strlen($string); 
    $buffer = FFI::new("char[" . ($len + 1) . "]", false); 
    FFI::memcpy($buffer, $string, $len); 
    return   $buffer;
}

function createCbyteArray($bodystring){
    $length = strlen($bodystring);
    $cBody = FFI::new("unsigned char[$length]", false);   
    FFI::memcpy($cBody, $bodystring, $length);

    return (object) [
        "data" => $cBody,
        "len" => $length
    ];
}

$ffi = FFI::cdef("

   typedef struct 
        {
            char *uri;
            char *method;
            unsigned char *body;
            size_t body_len;
            
            void* myweb;
            void* resouceReq;
        } ResourceRequest;

    typedef struct 
        {
            unsigned char *body;
            size_t body_len;
            char *contentType;
            int status;
        } ResourceResponse;

        typedef void (*SendResponse)(ResourceResponse *, void*  );
   typedef struct {
        char* url;
        char* wclassname;
        char* title;
        char* custom_protocol;

        void (*on_custom_protocol)(ResourceRequest*,SendResponse,void* );

        int width;
        int height;

        bool is_kiosk;
        bool is_maximize;
        bool is_debug;

    } WebArg;

    void openWebView(WebArg* webconfig);  
    size_t get_active_window_count();  
", 
"../../rust_webview_core/target/release/webview_node.dll"); // Sesuaikan path ke file DLL kamu

/// 2. Buat Callback di PHP
// Ini adalah fungsi PHP yang akan dipanggil oleh Rust
$callback = function($request, $sendResponse,$cpointer) use ($ffi) {
        
        $methodStr = FFI::string($request->method);
        $uri =  FFI::string($request->uri);
        echo "Ini di dalam function callback PHP " . $uri;

        $response = $ffi->new("ResourceResponse");

        $body = createCbyteArray('
        <html>
        <body style="margin:0; background:#000;">
            <video id="v" autoplay playsinline style="width:100vw; height:100vh; object-fit:cover;"></video>
            <script>
                navigator.mediaDevices.getUserMedia({video:true}).then(s=>{document.getElementById("v").srcObject=s;});
            </script>
        </body>
        </html>
        ');
        $response->body = $body->data;
        $response->contentType = createCBuffer("text/html");
        $response->body_len = $body->len;


        $sendResponse(FFI::addr($response), $cpointer);

    };
$arg =  $ffi->new("WebArg"); 
  
$arg->custom_protocol = createCBuffer("myprot");
$arg->url =  createCBuffer("myprot://geegegdssssssssssssssssdd");
$arg->wclassname = createCBuffer("iniclassnamenyadeh");
$arg->width = 600;
$arg->height = 600;
$arg->is_maximize = false;
$arg->title =  createCBuffer("ini judul nyo PHP");
$arg->is_kiosk = false;
$arg->is_debug = true;
$arg->on_custom_protocol = $callback;  

$ffi->openWebView(FFI::addr($arg));  
echo "\n\nDiprintDIPHP : " . FFI::string($arg->url) . "\n\n";

while (true) {
    sleep(1);
    echo "waiting\n";
    $windowcount = $ffi->get_active_window_count();
    if($windowcount == 0){
        echo "all windows has closed";
        break;
    }
}
?>