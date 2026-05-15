import { CustomProtocolResponse, keepLive, openWebView } from "tiny_webview/node"


openWebView({
    url : "mytest://index.html?mypost=ttt",
    customProtocol : "mytest", 
    height : 500,
    width : 800,
    isDebug : true,
    isKisok : false,
    isMaximize : false,
    title : "Test Title",
    customProtocolOnRequest : async (p)=>{
        let result = {
            body : Buffer.from("Halo ini tst DUlu ya " + p.uri),
        } as CustomProtocolResponse ;

        return result;
    }
});
 