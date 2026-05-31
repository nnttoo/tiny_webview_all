export declare function createIpcPipeName(ipcpath: string): string;
export declare function sendToIpc(ipcpath: string, data: Uint8Array): Promise<Uint8Array<ArrayBufferLike>>;
export interface CmdResponse {
    cmd: string;
    message: string;
}
export declare function sendIpcCmd(data: CmdResponse): Promise<CmdResponse>;
