
import path from "node:path"
import { copyNodeExe, deploy_exe, Platform } from "webview_ipc/builder"
import { buildServer } from "./build_es";
import { cp } from "node:fs/promises";


async function copyHtmlFolder(distfolder: string) { 
    let htmlfolder = path.join(__dirname, "../html"); 
    await cp(htmlfolder, path.join(distfolder,"html"), { recursive: true });
    console.log("Directory copied successfully!");
}

async function deploy() {
    await buildServer();

    let distFolder = path.join(__dirname, "../dist/");

    await copyNodeExe(path.join(distFolder, "/lib/node.exe")); 

    deploy_exe({
        exeFilePath: path.join(distFolder, "myApp.exe"),
        iconPath: path.join(__dirname, "../icon.png"),
        startCommand: ".\\lib\\node.exe ./lib/app.js",
        platform: Platform.Windows64
    });

    await copyHtmlFolder(distFolder);
}

deploy();