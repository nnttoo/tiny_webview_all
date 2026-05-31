export interface WebRequest {
    uri: string;
    method: string;
    body: Uint8Array;
    content_type: string;
    status: number;
}
export interface WebResponse {
    body: Buffer;
    content_type: string;
}
export interface CallWebRequest {
    bodyString: () => string;
    bodyJson: <T>() => T;
    bodyByte: () => Buffer;
    path: string;
    url: string;
    method: string;
}
export declare function startWebIpcServer(ipcpath: string, folderpath: string, callback?: (req: CallWebRequest) => Promise<WebResponse | null>): import("node:net").Server;
