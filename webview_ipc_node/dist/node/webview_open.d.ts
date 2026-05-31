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
export declare function openWebview(config: WebViewConfig): Promise<{
    close: () => Promise<void>;
}>;
