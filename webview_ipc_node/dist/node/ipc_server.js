"use strict";
// Haryanto 30 05 2026
// IPC Server implementation using native Node.js net streams with 4-byte length prefix
Object.defineProperty(exports, "__esModule", { value: true });
exports.createIpcServer = createIpcServer;
const net_1 = require("net");
const send_to_ipc_1 = require("./send_to_ipc");
function createIpcServer(ipcName, callback) {
    async function receiveFullByte(client, payload) {
        let data = Buffer.from("no callback found", "utf-8");
        if (callback != null) {
            data = await callback(payload);
        }
        // Haryanto 30 05 2026
        // send data length at first byte
        const lengthHeader = Buffer.alloc(4);
        lengthHeader.writeUInt32BE(data.length, 0);
        client.write(lengthHeader);
        client.write(data);
        client.end();
    }
    function handleClient(stream) {
        console.log("incoming connection");
        let buffer = Buffer.alloc(0);
        let expectedDataLength = -1;
        stream.on('data', (chunk) => {
            buffer = Buffer.concat([buffer, chunk]);
            if (expectedDataLength === -1) {
                if (buffer.length < 4) {
                    return;
                }
                expectedDataLength = buffer.readUInt32BE(0);
                buffer = buffer.subarray(4);
            }
            if (buffer.length >= expectedDataLength) {
                const actualPayload = buffer.subarray(0, expectedDataLength);
                stream.removeAllListeners('data');
                receiveFullByte(stream, actualPayload);
            }
        });
        stream.on('error', (err) => {
            console.error('[IPC Stream Error]:', err.message);
        });
        stream.on('end', () => {
            buffer = Buffer.alloc(0);
            expectedDataLength = -1;
        });
    }
    const pipePath = (0, send_to_ipc_1.createIpcPipeName)(ipcName);
    const server = (0, net_1.createServer)((stream) => {
        handleClient(stream);
    });
    server.listen(pipePath, () => {
    });
    server.on('error', (err) => {
        console.error('[IPC Server Error]:', err.message);
    });
    return server;
}
