import path from "node:path";
import { CustomProtocolRequest, CustomProtocolResponse } from "./libloader";
import { existsSync } from "node:fs";
import { readFile } from "node:fs/promises";

function getPathFromUrl(urlstring: string) {
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
} as { [key: string]: string };

function getContentType(urlpath: string) {
    let ext = path.extname(urlpath).toLocaleLowerCase();
    return mimeTypes[ext] || 'application/octet-stream';
}

export function createFileHandler(folderpath : string) {

    async function readFileByPath(uri: string): Promise<{
        body: Buffer<ArrayBuffer>,
        contentType : string
    }> { 
 

        let urlpath = getPathFromUrl(uri); 
        if (urlpath == null) throw "path is null"; 
        let actualFilePath = path.join(folderpath, urlpath);
         
        if (!existsSync(actualFilePath)) throw "file Not found";

        let body = await readFile(actualFilePath);
        return {
            body: body,
            contentType : getContentType(urlpath)
        };
    }

    return async function fileHandler(arg: CustomProtocolRequest) {
        let result = {
            status: 404,

        } as CustomProtocolResponse

 
        try {
            let fileResult = await readFileByPath(arg.uri);
            if(fileResult == null) throw "file not found"

            result.body = fileResult.body;
            result.content_type = fileResult.contentType;
            result.status = 200;

        } catch (error) {
            console.log(error);
        }

        if(result.body == null){
            result.body = Buffer.from("");
        }
 
        return result;
    }

}