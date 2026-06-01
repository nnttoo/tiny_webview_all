"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.selectFile = selectFile;
exports.selectFolder = selectFolder;
const send_to_ipc_1 = require("./send_to_ipc");
function selectFile(arg) {
    return (0, send_to_ipc_1.sendIpcCmd)({
        cmd: "select_file",
        message: JSON.stringify(arg)
    });
}
function selectFolder(arg) {
    return (0, send_to_ipc_1.sendIpcCmd)({
        cmd: "select_folder",
        message: JSON.stringify(arg)
    });
}
