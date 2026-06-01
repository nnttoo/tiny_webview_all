import { sendIpcCmd } from "./send_to_ipc"

interface FileType {
    file_name: String,
    ext: String[]
}
export interface FileSelectorArg {
    root_dir: String,
    file_types: FileType[]
}


export   function selectFile(arg: FileSelectorArg) {
    return sendIpcCmd({
        cmd: "select_file",
        message: JSON.stringify(arg)
    })
}