import { decode, encode } from "@msgpack/msgpack";
import { createIpcServer } from "./ipc_server";
import { createFileHandler, getPathFromUrl } from "./ipc_server_web_file";

export interface WebRequest {
    uri: string,
    method: string,
    body: Uint8Array,
    content_type: string,
    status: number,
}

export interface WebResponse {

    body: Buffer,
    content_type: string,
}

export interface CallWebRequest {
    bodyString: () => string,
    bodyJson: <T>() => T,
    bodyByte: () => Buffer,
    path: string,
    url: string,
    method: string,

}

export function startWebIpcServer(
    ipcpath: string,
    folderpath: string,
    callback?: (req: CallWebRequest) => Promise<WebResponse | null>
) {
    let filehandle = createFileHandler(folderpath)

    return createIpcServer(
        ipcpath,
        async (data) => {

            let webreq = decode<WebRequest>(data) as WebRequest;

            let response = await filehandle(webreq);
            if (response.status != 200 && callback != null) {
                try {
                    let responseFromCb = await callback({
                        bodyByte: () => {
                            return Buffer.from(webreq.body)
                        },
                        bodyString: () => {
                            return  Buffer.from(webreq.body).toString("utf-8")
                        },
                        bodyJson: <T>() => {
                            let obj = {} as T;
                            try {
                                let str = Buffer.from(webreq.body).toString()  
                                obj = JSON.parse(str)
                            } catch (error) {

                            }

                            return obj;
                        },
                        path: getPathFromUrl(webreq.uri),
                        url: webreq.uri,
                        method: webreq.method
                    });

                    if(responseFromCb != null){
                        response = {
                            body : responseFromCb.body?? "",
                            content_type : responseFromCb.content_type ?? "text/html",
                            method : webreq.method,
                            status : 200,
                            uri : webreq.uri
                        }
                    }
                } catch (error) {

                }


            } 


            // let response: WebRequest = {
            //     uri : "",
            //     body : Uint8Array.from(Buffer.from( "<h1>Ini dari Node yA","utf-8")),
            //     content_type : "",
            //     method : "",
            //     status : 200,

            // }

            let responseByte = encode(response);
            return responseByte;
        }
    )
} 