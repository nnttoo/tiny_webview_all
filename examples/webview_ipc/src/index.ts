import { createIpcServer } from "./ipc_server";
import { openWebview } from "./webview_open";



function sleep(n: number) {
    return new Promise((r, x) => {
        setTimeout(r, n);
    });
}

async function run() {
    let web : {close :()=>void } = { 
        close : ()=>{

        }
    }; 

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
            url: "https://harycodeworks.com",
            ipc_server: "mynodeipc"
        });

    } catch (error) {
        console.log("node js error");
    }

    let server =  createIpcServer("testipc");


    let hitung = 0;
    while (true) {
        await sleep(1000);
        hitung++;
        console.log(hitung);

        if (hitung == 5) {
            web.close();
        }

        if (hitung > 10) {
            console.log("App Exit from Node");
            break;
        }
    }

    console.log("close the server");
    server.close(); 

}

run();

