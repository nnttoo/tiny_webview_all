import { WebRequest } from "./ipc_server_web";
export declare function getPathFromUrl(urlstring: string): string;
export declare function createFileHandler(folderpath: string): (arg: WebRequest) => Promise<WebRequest>;
