import { createServer, Socket } from "net";
import { createIpcPipeName } from "./send_to_ipc";

export function create_ipc_server(
    ipc_name: string,
    callAfterByte: (data: string) => void
): void {
    const pipePath = createIpcPipeName(ipc_name);

    createServer((stream: Socket) => {
        console.log('[IPC] Client terhubung.');
        let buffer = Buffer.alloc(0);

        stream.on('data', (chunk: Buffer) => {
            buffer = Buffer.concat([buffer, chunk]);

            while (buffer.length >= 4) {
                const expectedLength = buffer.readUInt32BE(0);

                if (buffer.length < 4 + expectedLength) {
                    break;
                }
                const messageBuffer = buffer.subarray(4, 4 + expectedLength);
                const pesanFull = messageBuffer.toString('utf-8');

                callAfterByte(pesanFull);
                buffer = buffer.subarray(4 + expectedLength);
            }
        });

        stream.on('error', (err) => {
            console.error('[IPC Stream Error]:', err.message);
        });

        stream.on('end', () => {
            console.log('[IPC] Client memutuskan koneksi.');
            buffer = Buffer.alloc(0);
        });

    }).listen(pipePath, () => {
        console.log(`Server IPC berjalan.`);
    }).on('error', (err) => {
        console.error('[IPC Server Error]:', err.message);
    });
}