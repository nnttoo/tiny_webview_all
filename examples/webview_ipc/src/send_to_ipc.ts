
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
            console.log('✅ Terhubung ke Server Rust!');

            // Haryanto 30 05 2026
            // send data length at first byte
            const lengthHeader = Buffer.alloc(4);
            lengthHeader.writeUInt32BE(data.length, 0); 
            client.write(lengthHeader); 


            client.write(data);
        });

        let responseBuffer = Buffer.alloc(0);
        let expectedResponseLength = -1;

       // --- PROCESS RECEIVING RESPONSE ---
        client.on('data', (chunk: Buffer) => {
            // Append incoming chunk to the internal buffer
            responseBuffer = Buffer.concat([responseBuffer, chunk]);

            // If expected length is not set yet and we have at least 4 bytes for the header
            if (expectedResponseLength === -1 && responseBuffer.length >= 4) {
                expectedResponseLength = responseBuffer.readUInt32BE(0);
                // Strip the 4-byte header, keeping only the actual payload data
                responseBuffer = responseBuffer.subarray(4);
            }

            // If we know the expected length and the collected payload meets or exceeds it
            if (expectedResponseLength !== -1 && responseBuffer.length >= expectedResponseLength) {
                // Slice the exact length of the expected payload (handles any potential trailing data)
                const finalReply = responseBuffer.subarray(0, expectedResponseLength);
                
                client.end(); // Close connection since the communication is complete
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

