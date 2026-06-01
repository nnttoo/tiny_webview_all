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
exports.downloadFile = downloadFile;
const http = __importStar(require("http"));
const https = __importStar(require("https"));
const fs = __importStar(require("fs"));
const url_1 = require("url");
async function downloadFile(src, filePath) {
    return new Promise((resolve, reject) => {
        const url = new url_1.URL(src);
        const protocol = url.protocol === 'https:' ? https : http;
        // Gunakan flags 'w' untuk memastikan file dibuka untuk ditulisi
        const file = fs.createWriteStream(filePath, { flags: 'w' });
        protocol.get(src, (response) => {
            const { statusCode } = response;
            if (statusCode && statusCode >= 300 && statusCode < 400 && response.headers.location) {
                file.destroy(); // Hancurkan stream sebelum redirect
                return resolve(downloadFile(response.headers.location, filePath));
            }
            if (statusCode !== 200) {
                file.destroy();
                fs.unlink(filePath, () => { });
                return reject(new Error(`Download gagal! Status: ${statusCode}`));
            }
            response.pipe(file);
            // GANTI DISINI: Gunakan event 'close' bukan 'finish'
            file.on('close', () => {
                resolve(filePath);
            });
            file.on('error', (err) => {
                fs.unlink(filePath, () => { });
                reject(err);
            });
        }).on('error', (err) => {
            file.destroy();
            fs.unlink(filePath, () => { });
            reject(err);
        });
    });
}
