import { sendIpcCmd } from "./send_to_ipc"

 
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

export interface WebControl{
    close : ()=>Promise<void>;
    move : (left : number, top : number)=>Promise<void>
}

export async function openWebview(config: WebViewConfig) : Promise<WebControl>{
    let cmdRespnse = await sendIpcCmd({
        cmd: "openweb",
        message: JSON.stringify(config),
    });

    let win_id = Number(cmdRespnse.message);

    return {
        close: async () => {
            await sendIpcCmd({
                cmd: "closeweb",
                message: cmdRespnse.message
            })
        },

        move : async (left, top)=>{
             await sendIpcCmd({
                cmd: "move",
                message: JSON.stringify({win_id : win_id, left : left, top : top})
            })
        }
    }
}