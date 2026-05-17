interface FileType {
    file_name: String;
    ext: String[];
}
export interface FileSelectorArg {
    root_dir: String;
    file_types: FileType[];
}
export declare function openFileSelector(arg: FileSelectorArg): Promise<string>;
export declare function openFolderSelector(rootfolder: String): Promise<string>;
export {};
