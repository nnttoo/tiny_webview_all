
import { encode, decode } from "@msgpack/msgpack";
import { sendToIpc } from "./send_to_ipc";

function sleep(n: number) {
    return new Promise((r, x) => {
        setTimeout(r, n);
    });
}

export interface CmdResponse {
    cmd: string;
    message: string;
}
const pesanData: CmdResponse = {
    cmd: "openweb",
    message: "Halo dari Node.js TypeScript!",
};
const msgpackBytes: Uint8Array = encode(pesanData);
sendToIpc(msgpackBytes).then(async (data) => {
    console.log("sudah berhasil" + data.message);

    await sleep(1000);
    console.log("close window");
    sendToIpc(encode({
        cmd: "closeweb",
        message: data.message
    } as CmdResponse))
})