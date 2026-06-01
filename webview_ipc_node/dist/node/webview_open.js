"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.openWebview = openWebview;
const send_to_ipc_1 = require("./send_to_ipc");
async function openWebview(config) {
    let cmdRespnse = await (0, send_to_ipc_1.sendIpcCmd)({
        cmd: "openweb",
        message: JSON.stringify(config),
    });
    let win_id = Number(cmdRespnse.message);
    return {
        close: async () => {
            await (0, send_to_ipc_1.sendIpcCmd)({
                cmd: "closeweb",
                message: cmdRespnse.message
            });
        },
        move: async (left, top) => {
            await (0, send_to_ipc_1.sendIpcCmd)({
                cmd: "move",
                message: JSON.stringify({ win_id: win_id, left: left, top: top })
            });
        }
    };
}
