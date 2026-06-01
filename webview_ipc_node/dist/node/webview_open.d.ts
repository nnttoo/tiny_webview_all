export interface WebViewConfig {
    url: string;
    title: string;
    width: number;
    height: number;
    is_frameless: boolean;
    is_maximize: boolean;
    is_debug: boolean;
    is_resizable: boolean;
    is_fullscreen: boolean;
    is_always_ontop: boolean;
    ipc_server: string;
}
export interface WebControl {
    close: () => Promise<void>;
    move: (left: number, top: number) => Promise<void>;
    resize: (width: number, height: number) => Promise<void>;
    minimize: (minimize: boolean) => Promise<void>;
}
export declare function openWebview(config: WebViewConfig): Promise<WebControl>;
