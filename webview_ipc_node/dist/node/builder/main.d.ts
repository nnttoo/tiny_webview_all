export declare enum Platform {
    Windows32 = "Win32.exe",
    Windows64 = "Win64.exe",
    Linux32 = "Linux32",
    Linux64 = "Linux64",
    LinuxArm32 = "LinuxArm32",
    LinuxArm64 = "LinuxArm64",
    Mac64 = "MAC_64"
}
export declare function deploy_exe(arg: {
    exeFilePath: string;
    iconPath: string;
    startCommand: string;
    platform: Platform;
}): Promise<void>;
