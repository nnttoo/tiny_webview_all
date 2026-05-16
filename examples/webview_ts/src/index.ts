import path from "node:path";
import { createFileHandler, CustomProtocolResponse, openWebView } from "tiny_webview/node"


const htmlPath = path.join(__dirname, "../html");
const fileHandler = createFileHandler(htmlPath)

openWebView({
    url : "mytest://myapp.local/index.html",
    customProtocol : "mytest", 
    height : 500,
    width : 800,
    isDebug : true,
    isKisok : false,
    isMaximize : false,
    title : "Test Title",
    customProtocolOnRequest : async (p)=>{ 
        let result = await fileHandler(p); 

        return result;
    }
});
 