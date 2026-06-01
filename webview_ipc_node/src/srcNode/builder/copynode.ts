import { copyFile } from "node:fs/promises";

export async function copyNodeExe(dest: string) { 
    let inputNodeExe = process.execPath; 
    await copyFile(inputNodeExe, dest);
}
