interface FileType {
    file_name: String;
    ext: String[];
}
export interface FileSelectorArg {
    root_dir: String;
    file_types: FileType[];
}
export interface FolderSelectorArg {
    root_dir: String;
}
export declare function selectFile(arg: FileSelectorArg): Promise<import("./send_to_ipc").CmdResponse>;
export declare function selectFolder(arg: FolderSelectorArg): Promise<import("./send_to_ipc").CmdResponse>;
export {};
