"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.openFileSelector = openFileSelector;
exports.openFolderSelector = openFolderSelector;
const koffi_1 = __importDefault(require("koffi"));
const lib_loader_1 = require("./lib_loader");
const CallbackFileSelectorPtr = koffi_1.default.pointer("CallbackFileSelectorPtr", koffi_1.default.proto("void CallbackFileSelector(char *)"));
function openFileSelector(arg) {
    let lib = (0, lib_loader_1.loadLib)();
    let callback;
    let p = new Promise((r) => {
        callback = r;
    });
    const select_file = lib.func("select_file", "void", ["char*", CallbackFileSelectorPtr]);
    let ptrCb = koffi_1.default.register((s) => {
        callback(s);
    }, CallbackFileSelectorPtr);
    let argStr = JSON.stringify(arg);
    select_file(argStr, ptrCb);
    koffi_1.default.unregister(ptrCb);
    return p;
}
function openFolderSelector(rootfolder) {
    let lib = (0, lib_loader_1.loadLib)();
    let callback;
    let p = new Promise((r) => {
        callback = r;
    });
    const select_folder = lib.func("select_folder", "void", ["char*", CallbackFileSelectorPtr]);
    let ptrCb = koffi_1.default.register((s) => {
        callback(s);
    }, CallbackFileSelectorPtr);
    select_folder(rootfolder, ptrCb);
    koffi_1.default.unregister(ptrCb);
    return p;
}
