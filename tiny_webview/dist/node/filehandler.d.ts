import { CustomProtocolRequest, CustomProtocolResponse } from "./lib_open_webview";
export declare function createFileHandler(folderpath: string): (arg: CustomProtocolRequest) => Promise<CustomProtocolResponse>;
