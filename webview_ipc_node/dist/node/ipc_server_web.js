"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.startWebIpcServer = startWebIpcServer;
const msgpack_1 = require("@msgpack/msgpack");
const ipc_server_1 = require("./ipc_server");
const ipc_server_web_file_1 = require("./ipc_server_web_file");
function startWebIpcServer(ipcpath, folderpath, callback) {
    let filehandle = (0, ipc_server_web_file_1.createFileHandler)(folderpath);
    return (0, ipc_server_1.createIpcServer)(ipcpath, async (data) => {
        let webreq = (0, msgpack_1.decode)(data);
        let response = await filehandle(webreq);
        if (response.status != 200 && callback != null) {
            try {
                let responseFromCb = await callback({
                    bodyByte: () => {
                        return Buffer.from(webreq.body);
                    },
                    bodyString: () => {
                        return Buffer.from(webreq.body).toString("utf-8");
                    },
                    bodyJson: () => {
                        let obj = {};
                        try {
                            let str = Buffer.from(webreq.body).toString();
                            obj = JSON.parse(str);
                        }
                        catch (error) {
                        }
                        return obj;
                    },
                    path: (0, ipc_server_web_file_1.getPathFromUrl)(webreq.uri),
                    url: webreq.uri,
                    method: webreq.method
                });
                if (responseFromCb != null) {
                    response = {
                        body: responseFromCb.body ?? "",
                        content_type: responseFromCb.content_type ?? "text/html",
                        method: webreq.method,
                        status: 200,
                        uri: webreq.uri
                    };
                }
            }
            catch (error) {
            }
        }
        // let response: WebRequest = {
        //     uri : "",
        //     body : Uint8Array.from(Buffer.from( "<h1>Ini dari Node yA","utf-8")),
        //     content_type : "",
        //     method : "",
        //     status : 200,
        // }
        let responseByte = (0, msgpack_1.encode)(response);
        return responseByte;
    });
}
