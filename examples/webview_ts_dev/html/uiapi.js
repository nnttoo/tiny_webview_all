
// @ts-check
/// <reference path="./uiapi.d.ts" />

const uiapi_path = "/uiapi/";
const pathApi_Command = "command";
const pathAPi_CommandStop = "command_stop";
const pathAPi_CommandRead = "command_read";
const pathAPi_PrintLog = "printlog";

const END_STREAM_SIGNAL = "---ENDOFSTREAM---";
const delimiterBytes = new TextEncoder().encode(END_STREAM_SIGNAL);

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

    isDelimiterPresent(arr) {
        if (arr.length < delimiterBytes.length) return false;
        for (let i = 0; i < delimiterBytes.length; i++) {
            if (arr[arr.length - delimiterBytes.length + i] !== delimiterBytes[i]) {
                return false;
            }
        }
        return true;
    },

    async callUiApiCore(path, param) {
        let res = await fetch(uiApi.uiapi_path_join(path), {
            method: "POST",
            body: param
        });


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
                    let t = await uiApi.callUiApiBuffer(pathAPi_CommandRead, thread_id);
                    let arr = new Uint8Array(t);

                    if (uiApi.isDelimiterPresent(arr)) { 
                        stopCommand();
                        return;
                    }

                    f(arr);
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