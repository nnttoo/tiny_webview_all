export interface CustomProtocolResponse {
    body: Buffer<ArrayBuffer>;
    content_type: string;
    status: number;
}
export interface CustomProtocolRequest {
    uri: string;
    method: string;
    body: Buffer<ArrayBuffer>;
}
export interface WebConfig {
    url: string;
    customProtocol?: string;
    customProtocolOnRequest?: (p: CustomProtocolRequest) => Promise<CustomProtocolResponse>;
    title: string;
    width: number;
    height: number;
    isKisok: boolean;
    isMaximize: boolean;
    isDebug: boolean;
}
export declare function openWebView(arg: WebConfig): Promise<void>;
