
import { decode } from '@msgpack/msgpack'; 
import * as net from 'node:net';
import { CmdResponse } from '.';

export function sendToIpc(data: Uint8Array) {
 
    return new Promise<CmdResponse>((resolve, x) => {
        const PIPE_PATH: string = process.platform === 'win32'
            ? '\\\\.\\pipe\\my-ipc'
            : '\0my-ipc';
 
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
            let decodedData  = {} as CmdResponse;

            try { 
                const uint8Array = new Uint8Array(completeBuffer);  
                decodedData = decode(uint8Array) as CmdResponse; 
                
            } catch (error) { 
            }

            resolve(decodedData);
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

