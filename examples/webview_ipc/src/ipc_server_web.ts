import { decode, encode } from "@msgpack/msgpack";
import { createIpcServer } from "./ipc_server";
import { createFileHandler } from "./ipc_server_web_file";

export interface WebRequest {
    uri: string,
    method: string,
    body: Uint8Array,
    content_type: string,
    status: number,
}

export function startWebIpcServer(ipcpath: string, folderpath : string) {
    let filehandle = createFileHandler(folderpath)

    return createIpcServer(
        ipcpath,
        async (data) => {

            let webreq = decode<WebRequest>(data) as WebRequest;

            let fromfile = await filehandle(webreq);


            // let response: WebRequest = {
            //     uri : "",
            //     body : Uint8Array.from(Buffer.from( "<h1>Ini dari Node yA","utf-8")),
            //     content_type : "",
            //     method : "",
            //     status : 200,

            // }

            let responseByte = encode(fromfile);
            return responseByte;
        }
    )
} 