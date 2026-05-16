import { CustomProtocolRequest, CustomProtocolResponse } from "./libloader";
export declare function createFileHandler(folderpath: string): (arg: CustomProtocolRequest) => Promise<CustomProtocolResponse>;
