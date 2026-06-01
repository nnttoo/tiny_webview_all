"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __exportStar = (this && this.__exportStar) || function(m, exports) {
    for (var p in m) if (p !== "default" && !Object.prototype.hasOwnProperty.call(exports, p)) __createBinding(exports, m, p);
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.Platform = void 0;
exports.deploy_exe = deploy_exe;
const node_fs_1 = require("node:fs");
const promises_1 = require("node:fs/promises");
const node_path_1 = __importDefault(require("node:path"));
const sharp_1 = __importDefault(require("sharp"));
const sharp_ico_1 = __importDefault(require("sharp-ico"));
const rcedit_1 = __importDefault(require("rcedit"));
const downloader_1 = require("../util/downloader");
__exportStar(require("./copynode"), exports);
var Platform;
(function (Platform) {
    Platform["Windows32"] = "Win32.exe";
    Platform["Windows64"] = "Win64.exe";
    Platform["Linux32"] = "Linux32";
    Platform["Linux64"] = "Linux64";
    Platform["LinuxArm32"] = "LinuxArm32";
    Platform["LinuxArm64"] = "LinuxArm64";
    Platform["Mac64"] = "MAC_64";
})(Platform || (exports.Platform = Platform = {}));
async function checkFile(platform) {
    let fileName = "webview_ipc_" + platform;
    let libpath = node_path_1.default.join(__dirname, "../../../lib", fileName);
    if ((0, node_fs_1.existsSync)(libpath))
        return libpath;
    // if not exist try check on local
    await (0, promises_1.mkdir)(node_path_1.default.dirname(libpath), { recursive: true });
    if (platform == Platform.Windows64) {
        // only for local dev machine
        let buildedPath = node_path_1.default.join(__dirname, "../../../../webview_ipc/target/release/webview_ipc.exe");
        if ((0, node_fs_1.existsSync)(buildedPath)) {
            await (0, promises_1.copyFile)(buildedPath, libpath);
        }
    }
    if (!(0, node_fs_1.existsSync)(libpath)) {
        console.log("Doanload binary " + fileName);
        let urltoDownload = "https://github.com/nnttoo/webview_ipc/releases/download/v1.0.5/webview_ipc_" + platform;
        await (0, downloader_1.downloadFile)(urltoDownload, libpath);
        console.log("download file done");
    }
    return libpath;
}
async function copyExeFile(exe_dest_path, platform) {
    let from = await checkFile(platform);
    if (!(0, node_fs_1.existsSync)(from)) {
        console.log("file not exists");
        return;
    }
    let distDir = node_path_1.default.dirname(exe_dest_path);
    await (0, promises_1.mkdir)(distDir, { recursive: true });
    await (0, promises_1.copyFile)(from, exe_dest_path);
}
async function changeIcon(arg) {
    let outputIcon = node_path_1.default.join(node_path_1.default.dirname(arg.exePath), "app.ico");
    const sizes = [16, 32, 48, 256]; // Ukuran standar untuk rcedit
    const images = await Promise.all(sizes.map(size => (0, sharp_1.default)(arg.iconPath)
        .resize(size, size)
        .toFormat('png')));
    await sharp_ico_1.default.sharpsToIco(images, outputIcon);
    await (0, rcedit_1.default)(arg.exePath, {
        icon: outputIcon,
    });
    // copy icon file
    let pngPath = node_path_1.default.join(node_path_1.default.dirname(arg.exePath), "icon.png");
    await (0, promises_1.copyFile)(arg.iconPath, pngPath);
}
async function deploy_exe(arg) {
    await copyExeFile(arg.exeFilePath, arg.platform);
    if (arg.platform == Platform.Windows32 || arg.platform == Platform.Windows64) {
        await changeIcon({
            exePath: arg.exeFilePath,
            iconPath: arg.iconPath
        });
    }
    let cmdFile = node_path_1.default.join(node_path_1.default.dirname(arg.exeFilePath), "index_cmd");
    await (0, promises_1.writeFile)(cmdFile, arg.startCommand);
}
