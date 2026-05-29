import { sendIpcCmd } from "./send_to_ipc";

export interface WebViewConfig {
    url: string,
    title: string,
    width: number,
    height: number,
    is_frameless: boolean,
    is_maximize: boolean,
    is_debug: boolean,
    is_resizable : boolean,
    is_fullscreen : boolean,
    is_always_ontop : boolean,
    ipc_server : string,
}

export async function openWebview(config: WebViewConfig) {
    let cmdRespnse = await sendIpcCmd({
        cmd: "openweb",
        message: JSON.stringify(config),
    })

    return {
        close: async () => {
            await sendIpcCmd({
                cmd: "closeweb",
                message: cmdRespnse.message
            })
        }
    }
}