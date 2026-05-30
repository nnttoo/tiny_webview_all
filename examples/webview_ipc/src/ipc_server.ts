// Haryanto 30 05 2026
// IPC Server implementation using native Node.js net streams with 4-byte length prefix

import { createServer, Server, Socket } from 'net';
import { createIpcPipeName } from './send_to_ipc';
import { buffer } from 'stream/consumers';

/**
 * Processes the completely assembled raw byte packet received via IPC.
 * @param payload The complete raw byte buffer (equivalent to Vec<u8> from Rust)
 */
function receiveFullByte(client: Socket, payload: Buffer,): void {

    let str = payload.toString("utf-8");
    console.log(str);

    console.log("send to server");

    let data = Buffer.from("ini balasan", "utf-8");
    // Haryanto 30 05 2026
    // send data length at first byte
    const lengthHeader = Buffer.alloc(4);
    lengthHeader.writeUInt32BE(data.length, 0);
    client.write(lengthHeader); 
    client.write(data);
    client.end();
}

function handleClient(stream: Socket): void {
    console.log("incoming connection");
    let buffer: Buffer = Buffer.alloc(0);
    let expectedDataLength: number = -1;

    stream.on('data', (chunk: Buffer) => {
        buffer = Buffer.concat([buffer, chunk]);

        if (expectedDataLength === -1) {
            if (buffer.length < 4) {
                return;
            }

            expectedDataLength = buffer.readUInt32BE(0);
            buffer = buffer.subarray(4);
        }

        if (buffer.length >= expectedDataLength) {
            const actualPayload: Buffer = buffer.subarray(0, expectedDataLength);
            
            stream.removeAllListeners('data');
            
            receiveFullByte(stream, actualPayload);
        }
    });

    stream.on('error', (err: Error) => {
        console.error('[IPC Stream Error]:', err.message);
    });

    stream.on('end', () => {
        buffer = Buffer.alloc(0);
        expectedDataLength = -1;
    });
}

export function createIpcServer(ipcName: string): Server {
    const pipePath: string = createIpcPipeName(ipcName);

    const server: Server = createServer((stream: Socket) => {
        handleClient(stream);
    });

    server.listen(pipePath, () => {
    });

    server.on('error', (err: Error) => {
        console.error('[IPC Server Error]:', err.message);
    });

    return server;
}