
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

    async callUiApiCore(path, param) {
        let res = await fetch(uiApi.uiapi_path_join(path), {
            method: "POST",
            body: param
        });

        if (!res.ok) {
            let txt = await res.text();
            throw (txt);
        }


        return res;
    },

    async callUiApi(path, param) {
        let res = await uiApi.callUiApiCore(path, param);
        let txt = await res.text();
        return txt;
    },

    async callUiApiBuffer(path, param) {
        let res = await uiApi.callUiApiCore(path, param);
        let blob = await res.arrayBuffer();
        return blob;
    },

    uiLog(dd) {
        return uiApi.callUiApi(pathAPi_PrintLog, dd);
    },

    async callCommand(cmdname) {
        let thread_id = await uiApi.callUiApi(pathApi_Command, cmdname);
        let keepLive = true;

        function stopCommand() {
            if (thread_id == "") return;
            keepLive = false;
            uiApi.callUiApi(pathAPi_CommandStop, thread_id);
        }

        return {
            async setOnData(f) {
                while (keepLive) {
                    try {
                        let t = await uiApi.callUiApiBuffer(pathAPi_CommandRead, thread_id);
                        let arr = new Uint8Array(t);
                        f(arr);
                    } catch (error) {
                        keepLive = false;
                    } 
                }
            },
            stopCommand
        }
    }
};





let decoder = new TextDecoder("utf-8");

async function test() {
    uiApi.uiLog("ini test dulu ya boss");
    let json = await uiApi.callCommand("ping");
    json.setOnData((data) => {
        if (data.length == 0) return;
        let textString = decoder.decode(data);
        uiApi.uiLog("ini datanya :" + textString);
    })

    //await uiApi.sleep(20000);
    //json.stopCommand();
}

test();