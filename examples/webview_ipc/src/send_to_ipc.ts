
import { decode, encode } from '@msgpack/msgpack';
import * as net from 'node:net';

export function sendToIpc(ipcpath: string, data: Uint8Array) {

    return new Promise<Uint8Array>((resolve, onErr) => {
        const PIPE_PATH: string = process.platform === 'win32'
            ? '\\\\.\\pipe\\' + ipcpath
            : '\0' + ipcpath;

        const client: net.Socket = net.createConnection(PIPE_PATH, (): void => {
            console.log('✅ Terhubung ke Server Rust!');
            client.write(data);
        });

        const chunks: Buffer[] = [];

        client.on('data', (chunk: Buffer): void => {
            chunks.push(chunk);
        });

        client.on('end', (): void => {
            const completeBuffer = Buffer.concat(chunks);
            let uint8Array: Uint8Array | null = null;

            try {
                uint8Array = new Uint8Array(completeBuffer);

            } catch (error) {
                onErr(error);
            }

            if (uint8Array != null) {
                resolve(uint8Array);
            } else {
                onErr("data null");
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

let ipcpath = process.env.IPCNAME? process.env.IPCNAME : "err" ;

export async function sendIpcCmd(data: CmdResponse) {
    let arrData = encode(data);

    let responseData = await sendToIpc(ipcpath, arrData);
    let cmdResponse = decode(responseData) as CmdResponse;
    return cmdResponse;
}

