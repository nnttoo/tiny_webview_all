import { encode } from "@msgpack/msgpack";
import { createIpcServer } from "./ipc_server";

export interface WebRequest {
    uri: string,
    method: string,
    body: Uint8Array,
    content_type: string,
}

export function startWebIpcServer(ipcpath : string){
    return createIpcServer(
        ipcpath,
        async (data)=>{


            let response: WebRequest = {
                uri : "",
                body : Uint8Array.from(Buffer.from( "<h1>Ini dari Node yA","utf-8")),
                content_type : "",
                method : "",
                
            }

            let responseByte = encode(response);
            return responseByte;    
        }
    )
} 