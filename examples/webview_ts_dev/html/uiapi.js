
// @ts-check
/// <reference path="./uiapi.d.ts" />

const uiapi_path = "/uiapi/";
const pathApi_Command = "command";
const pathAPi_CommandStop = "command_stop";
const pathAPi_CommandRead = "command_read";
const pathAPi_PrintLog = "printlog";
/** @type {import("./uiapi").UiAPi} */
const uiApi = {


    uiapi_path_join(str) {
        if (str.startsWith("/")) {
            str = str.substring(1);
        }

        return uiapi_path + str;
    },
    sleep(n) {
        return new Promise((r, x) => {
            setTimeout(r, n);
        })
    },

    async callUiApi(path, param) {
        let res = await fetch(uiApi.uiapi_path_join(path), {
            method: "POST",
            body: param
        });
 

        let txt = await res.text();
        return txt;
    },

    uiLog(dd) {
        return uiApi.callUiApi(pathAPi_PrintLog, dd);
    },

    async callCommand(cmdname) {
        let thread_id = await uiApi.callUiApi(pathApi_Command, cmdname);
        let keepLive = true;

        return {
            async setOnData(f) {
                while (keepLive) {
                    let t = await uiApi.callUiApi(pathAPi_CommandRead,thread_id);
                    f(t);
                    await uiApi.sleep(1000);
                }
            },
            stopCommand() {
                if (thread_id == "") return;
                keepLive = false;
                uiApi.callUiApi(pathAPi_CommandStop, thread_id);
            }
        }
    }
};






async function test() {
    uiApi.uiLog("ini test dulu ya boss");
    let json = await uiApi.callCommand("ping");
    json.setOnData((data)=>{
        uiApi.uiLog("ini datanya :" + data);
    })

    await uiApi.sleep(2000);
    json.stopCommand();
}

test();