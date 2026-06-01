
import path from "node:path"
import { deploy_exe, Platform } from "webview_ipc/builder"
import { buildServer } from "./build_es";


async function deploy() {
    await buildServer();


    deploy_exe({
        exeFilePath: path.join(__dirname, "../dist/myApp.exe"),
        iconPath : path.join(__dirname,"../icon.png"),
        startCommand : "dir",
        platform : Platform.Windows32
    });
}

deploy();