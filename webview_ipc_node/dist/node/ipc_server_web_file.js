"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.getPathFromUrl = getPathFromUrl;
exports.createFileHandler = createFileHandler;
const node_path_1 = __importDefault(require("node:path"));
const node_fs_1 = require("node:fs");
const promises_1 = require("node:fs/promises");
function getPathFromUrl(urlstring) {
    let urlObj = new URL(urlstring);
    return urlObj.pathname;
}
const mimeTypes = {
    '.html': 'text/html',
    '.htm': 'text/html',
    '.css': 'text/css',
    '.js': 'application/javascript',
    '.json': 'application/json',
    '.png': 'image/png',
    '.jpg': 'image/jpeg',
    '.jpeg': 'image/jpeg',
    '.gif': 'image/gif',
    '.svg': 'image/svg+xml',
    '.ico': 'image/x-icon',
    '.txt': 'text/plain',
    '.pdf': 'application/pdf',
    '.zip': 'application/zip',
    '.mp3': 'audio/mpeg',
    '.mp4': 'video/mp4'
};
function getContentType(urlpath) {
    let ext = node_path_1.default.extname(urlpath).toLocaleLowerCase();
    return mimeTypes[ext] || 'application/octet-stream';
}
function createFileHandler(folderpath) {
    async function readFileByPath(uri) {
        let urlpath = getPathFromUrl(uri);
        if (urlpath == null)
            throw "path is null";
        let actualFilePath = node_path_1.default.join(folderpath, urlpath);
        if (!(0, node_fs_1.existsSync)(actualFilePath))
            throw "file Not found";
        let body = await (0, promises_1.readFile)(actualFilePath);
        return {
            body: body,
            contentType: getContentType(urlpath)
        };
    }
    return async function fileHandler(arg) {
        let result = {
            status: 404,
            uri: "",
            body: Buffer.from("", "utf-8"),
            content_type: "",
            method: "",
        };
        try {
            let fileResult = await readFileByPath(arg.uri);
            if (fileResult == null)
                throw "file not found";
            result.body = fileResult.body;
            result.content_type = fileResult.contentType;
            result.status = 200;
        }
        catch (error) {
            console.log(error);
        }
        if (result.body == null) {
            result.body = Buffer.from("");
        }
        return result;
    };
}
