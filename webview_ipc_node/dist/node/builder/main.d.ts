export declare enum Platform {
    Windows32 = "WINDOWS_32",
    Windows64 = "WINDOWS_64",
    Linux32 = "LINUX_32",
    Linux64 = "LINUX_64",
    LinuxArm32 = "LINUX_ARM32",
    LinuxArm64 = "LINUX_ARM64"
}
export declare function deploy_exe(arg: {
    exeFilePath: string;
    iconPath: string;
    startCommand: string;
    platform: Platform;
}): Promise<void>;
