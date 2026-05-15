"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.sleep = sleep;
exports.createCBuffer = createCBuffer;
function sleep(n) {
    return new Promise((r, x) => {
        setTimeout(() => { r(null); }, n);
    });
}
function createCBuffer(str) {
    return Buffer.from(str + "\0", "utf8");
}
