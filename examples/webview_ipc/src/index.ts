import path from "node:path";
import { openWebview, startWebIpcServer, WebControl, WebResponse } from "webview_ipc"

let ipcpath = process.env.IPCNAME ? process.env.IPCNAME : "err";

export interface FParam {
    cmd: string,
    params: any
}

async function run() {
    let web: WebControl

    let myipcpath = ipcpath + "mynodeipc";

    startWebIpcServer(
        myipcpath,
        path.join(__dirname, "../html"),
        async (req) => {
            if (req.path != "/controlwindow") {
                return null;
            }

            let simple_result = (msg: string) => {
                return {
                    body: Buffer.from(msg),
                    content_type: "application/json"
                }
            } 
            let json = req.bodyJson<FParam>();

            if (json.cmd == null) return null;

            if (json.cmd == "close") {

                web.close();
                return simple_result("ok");
            }

            if (json.cmd == "move") {
                let arg = json.params as { top: number, left: number };
                web.move(arg.left, arg.top);
               
                return simple_result("ok");
            }

            if (json.cmd == "resize") {
                let arg = json.params as { width: number, height: number };
                web.resize(arg.width,arg.height);
                return simple_result("ok");

            }

            if (json.cmd == "minimize") { 
                web.minimize(json.params as boolean);
                return simple_result("ok");

            }

            console.log(json.cmd);
            console.log(json.params);

            return null;

        }
    );

    try {
        web = await openWebview({
            height: 600,
            width: 1000,
            is_debug: true,
            is_frameless: false,
            is_maximize: false,
            is_resizable: true,
            is_always_ontop: false,
            is_fullscreen: false,
            title: "My Web Title",
            url: myipcpath + "://myapp.local/index.html",
            ipc_server: myipcpath
        });



    } catch (error) {
        console.log("node js error");
    }



    // let hitung = 0;
    // while (true) {
    //     await sleep(1000);
    //     hitung++;
    //     console.log(hitung);

    //     if (hitung == 5) {
    //         web.close();
    //     }

    //     if (hitung > 10) {
    //         console.log("App Exit from Node");
    //         break;
    //     }
    // }

    // console.log("close the server");
    // server.close();

}

run();

