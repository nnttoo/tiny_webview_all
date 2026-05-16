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
export interface TsOnlyWindowControl {
    close: () => void;
    move: (left: number, top: number) => void;
    resize: (height: number, width: number) => void;
    maximize: (ismax: boolean) => void;
    minimize: (ismax: boolean) => void;
}
export interface TsOnlyWebConfig {
    url: string;
    customProtocol?: string;
    customProtocolOnRequest?: (p: CustomProtocolRequest) => Promise<CustomProtocolResponse>;
    title: string;
    width: number;
    height: number;
    isKisok: boolean;
    isMaximize: boolean;
    isDebug: boolean;
    onwindowOpenned: (p: TsOnlyWindowControl) => void;
}
export declare function openWebView(arg: TsOnlyWebConfig): Promise<void>;
