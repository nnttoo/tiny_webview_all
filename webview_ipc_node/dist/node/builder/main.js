"use strict";
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
var Platform;
(function (Platform) {
    Platform["Windows32"] = "WINDOWS_32";
    Platform["Windows64"] = "WINDOWS_64";
    Platform["Linux32"] = "LINUX_32";
    Platform["Linux64"] = "LINUX_64";
    Platform["LinuxArm32"] = "LINUX_ARM32";
    Platform["LinuxArm64"] = "LINUX_ARM64"; // Fixed typo from 'argm64' to 'Arm64'
})(Platform || (exports.Platform = Platform = {}));
async function copyExeFile(exe_dest_path, platform) {
    let from = (() => {
        if (platform == Platform.Windows64) {
            return node_path_1.default.join(__dirname, "../../../../webview_ipc/target/release/webview_ipc.exe");
        }
        return "notexit";
    })();
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
}
async function deploy_exe(arg) {
    await copyExeFile(arg.exeFilePath, arg.platform);
    await changeIcon({
        exePath: arg.exeFilePath,
        iconPath: arg.iconPath
    });
    let cmdFile = node_path_1.default.join(node_path_1.default.dirname(arg.exeFilePath), "index_cmd");
    await (0, promises_1.writeFile)(cmdFile, arg.startCommand);
}
