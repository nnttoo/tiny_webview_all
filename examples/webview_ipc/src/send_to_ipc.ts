
import { decode, encode } from '@msgpack/msgpack';
import * as net from 'node:net';

export function createIpcPipeName(ipcpath: string) {
    const PIPE_PATH: string = process.platform === 'win32'
        ? '\\\\.\\pipe\\' + ipcpath
        : '\0' + ipcpath;
    return PIPE_PATH;
}


export function sendToIpc(ipcpath: string, data: Uint8Array) {

    return new Promise<Uint8Array>((resolve, onErr) => {

        const PIPE_PATH = createIpcPipeName(ipcpath);
        const client: net.Socket = net.createConnection(PIPE_PATH, (): void => {
            console.log('✅ Call Rust IPC');

            // Haryanto 30 05 2026
            // send data length at first byte
            const lengthHeader = Buffer.alloc(4);
            lengthHeader.writeUInt32BE(data.length, 0); 
            client.write(lengthHeader); 


            client.write(data);
        });

        let responseBuffer = Buffer.alloc(0);
        let expectedResponseLength = -1;
 
        client.on('data', (chunk: Buffer) => { 
            responseBuffer = Buffer.concat([responseBuffer, chunk]); 
            if (expectedResponseLength === -1 && responseBuffer.length >= 4) {
                expectedResponseLength = responseBuffer.readUInt32BE(0); 
                responseBuffer = responseBuffer.subarray(4);
            } 
            if (expectedResponseLength !== -1 && responseBuffer.length >= expectedResponseLength) { 
                const finalReply = responseBuffer.subarray(0, expectedResponseLength);
                
                client.end();  
                resolve(finalReply);
            }
        });
         

        client.on('error', (err: NodeJS.ErrnoException): void => {
            if (err.code === 'ENOENT') {
                console.error('❌ Gagal terhubung: Server Rust belum aktif atau nama pipe salah.');
            } else {
                console.error('💥 Terjadi kesalahan koneksi:', err.message);
            }
        });
    })

}

export interface CmdResponse {
    cmd: string;
    message: string;
}

let ipcpath = process.env.IPCNAME ? process.env.IPCNAME : "err";

export async function sendIpcCmd(data: CmdResponse) {
    let arrData = encode(data); 
    let responseData = await sendToIpc(ipcpath, arrData); 
    let cmdResponse = decode(responseData) as CmdResponse;
    return cmdResponse;
}

