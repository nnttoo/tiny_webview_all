"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.copyNodeExe = copyNodeExe;
const promises_1 = require("node:fs/promises");
async function copyNodeExe(dest) {
    let inputNodeExe = process.execPath;
    await (0, promises_1.copyFile)(inputNodeExe, dest);
}
