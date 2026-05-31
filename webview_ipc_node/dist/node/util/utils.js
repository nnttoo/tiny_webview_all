"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.sleep = sleep;
function sleep(n) {
    return new Promise((r, x) => {
        setTimeout(r, n);
    });
}
