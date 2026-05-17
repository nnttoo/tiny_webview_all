import koffi from 'koffi';
import { loadLib } from "./lib_loader";


const CallbackFileSelectorPtr = koffi.pointer(
    "CallbackFileSelectorPtr",
    koffi.proto("void CallbackFileSelector(char *)")
)

interface FileType {
    file_name: String,
    ext: String[]
}
export interface FileSelectorArg {
    root_dir: String,
    file_types: FileType[]
}

export function openFileSelector(arg : FileSelectorArg) {
    let lib = loadLib();

    let callback : (str : string)=>void;
    let p = new Promise<string>((r)=>{
        callback = r;
    })



    const select_file = lib.func("select_file", "void", ["char*",CallbackFileSelectorPtr]);
    let ptrCb = koffi.register((s:string)=>{ 
        callback(s); 
    }, CallbackFileSelectorPtr);

    let argStr = JSON.stringify(arg);
    select_file(argStr,ptrCb);
    koffi.unregister(ptrCb);

    return p;
}


export function openFolderSelector(rootfolder : String) {
    let lib = loadLib();

    let callback : (str : string)=>void;
    let p = new Promise<string>((r)=>{
        callback = r;
    }) 

    const select_folder = lib.func("select_folder", "void", ["char*",CallbackFileSelectorPtr]);
    let ptrCb = koffi.register((s:string)=>{ 
        callback(s); 
    }, CallbackFileSelectorPtr);
 
    select_folder(rootfolder,ptrCb);
    koffi.unregister(ptrCb);

    return p;
}