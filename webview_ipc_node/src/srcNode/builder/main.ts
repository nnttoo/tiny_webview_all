import { existsSync } from "node:fs";
import { copyFile, mkdir, writeFile } from "node:fs/promises";
import path from "node:path";
import sharp from "sharp"
import ico from "sharp-ico" 
import rcedit from "rcedit";

export enum Platform {
    Windows32 = "WINDOWS_32",
    Windows64 = "WINDOWS_64",
    Linux32 = "LINUX_32",
    Linux64 = "LINUX_64",
    LinuxArm32 = "LINUX_ARM32",
    LinuxArm64 = "LINUX_ARM64" // Fixed typo from 'argm64' to 'Arm64'
}

async function copyExeFile(exe_dest_path: string, platform : Platform) {

    let from = (()=>{
        if(platform == Platform.Windows64){
            return path.join(__dirname, "../../../../webview_ipc/target/release/webview_ipc.exe");
        }

        return "notexit"
    })();
    



    if (!existsSync(from)) {
        console.log("file not exists");
        return;
    }

    let distDir = path.dirname(exe_dest_path);

    await mkdir(distDir, { recursive: true })

    await copyFile(from, exe_dest_path);

}

async function changeIcon(arg: {
    iconPath: string,
    exePath: string,
}) {

    let outputIcon = path.join(path.dirname(arg.exePath), "app.ico");
    const sizes = [16, 32, 48, 256]; // Ukuran standar untuk rcedit
    const images = await Promise.all(sizes.map(size =>
        sharp(arg.iconPath)
            .resize(size, size)
            .toFormat('png')
    ));
    await ico.sharpsToIco(images, outputIcon);

    await rcedit(arg.exePath, {
        icon: outputIcon,
    });

}

export async function deploy_exe(arg: {
    exeFilePath: string,
    iconPath : string,
    startCommand : string,
    platform : Platform
}) {
    await copyExeFile(arg.exeFilePath, arg.platform);
    await changeIcon({
        exePath : arg.exeFilePath,
        iconPath : arg.iconPath
    });

    let cmdFile = path.join( path.dirname(arg.exeFilePath),"index_cmd");
    await writeFile(cmdFile, arg.startCommand);
}