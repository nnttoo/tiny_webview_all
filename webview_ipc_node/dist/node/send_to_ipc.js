"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.createIpcPipeName = createIpcPipeName;
exports.sendToIpc = sendToIpc;
exports.sendIpcCmd = sendIpcCmd;
const msgpack_1 = require("@msgpack/msgpack");
const net = __importStar(require("node:net"));
function createIpcPipeName(ipcpath) {
    const PIPE_PATH = process.platform === 'win32'
        ? '\\\\.\\pipe\\' + ipcpath
        : '\0' + ipcpath;
    return PIPE_PATH;
}
function sendToIpc(ipcpath, data) {
    return new Promise((resolve, onErr) => {
        const PIPE_PATH = createIpcPipeName(ipcpath);
        const client = net.createConnection(PIPE_PATH, () => {
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
        client.on('data', (chunk) => {
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
        client.on('error', (err) => {
            if (err.code === 'ENOENT') {
                console.error('❌ Gagal terhubung: Server Rust belum aktif atau nama pipe salah.');
            }
            else {
                console.error('💥 Terjadi kesalahan koneksi:', err.message);
            }
        });
    });
}
let ipcpath = process.env.IPCNAME ? process.env.IPCNAME : "err";
async function sendIpcCmd(data) {
    let arrData = (0, msgpack_1.encode)(data);
    let responseData = await sendToIpc(ipcpath, arrData);
    let cmdResponse = (0, msgpack_1.decode)(responseData);
    return cmdResponse;
}
