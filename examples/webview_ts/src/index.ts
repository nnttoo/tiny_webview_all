import path from "node:path";
import { createFileHandler, CustomProtocolRequest, CustomProtocolResponse, openFileSelector, openFolderSelector, openWebView, TsOnlyWindowControl } from "tiny_webview/node"
export interface FParam {
    cmd: string,
    params: any
}

function openDllWebView() {

    const htmlPath = path.join(__dirname, "../html");
    const fileHandler = createFileHandler(htmlPath);
    let windowController: TsOnlyWindowControl;

    async function apiHandler(p: CustomProtocolRequest) {
        if (p.method != "POST") return null;

        let url = new URL(p.uri);

        if (url.pathname != "/controlwindow") return;

        let body = p.body.toString();
        let bodyObj = JSON.parse(body) as FParam;

        console.log(bodyObj);

        if (bodyObj.cmd == "close") {
            windowController.close();
            return;
        }

        if (bodyObj.cmd == "move") {
            let param = bodyObj.params;
            windowController.move(param.left, param.top);
            return;
        }

        if (bodyObj.cmd == "resize") {
            let param = bodyObj.params;
            windowController.resize(param.width, param.height);
            return;
        }

        if (bodyObj.cmd == "maximize") {
            let param = bodyObj.params;
            windowController.maximize(param);
            return;
        }

        if (bodyObj.cmd == "minimize") {
            let param = bodyObj.params;
            windowController.minimize(param);
            return;
        }

        if (bodyObj.cmd == "openfile") {
            let filepath = await openFileSelector({
                file_types: [
                    {
                        ext: ['md', "txt", "zip"],
                        file_name: "Text File"
                    }
                ],
                root_dir: "D:\\hhhhhhhhhhhhhhhhhhh"
            });
            return filepath;
        }

        if (bodyObj.cmd == "openfolder") {
            let filepath = await openFolderSelector("D:\\hhhhhhhhhhhhhhhhhhh");
            return filepath;
        }

    }

    openWebView({
        url: "mytest://myapp.local/index.html",
        customProtocol: "mytest",
        height: 500,
        width: 800,
        isDebug: true,
        isKisok: false,
        isMaximize: false,
        title: "Test Title",
        customProtocolOnRequest: async (p) => {
            let result = await fileHandler(p);

            if (result.status == 404) {
                let apiResult = await apiHandler(p);
                if (apiResult != null) {
                    result.body = Buffer.from(apiResult);
                    result.content_type = "text/plain"
                    result.status = 200;
                }

            }

            return result;
        },
        onwindowOpenned: (p) => {
            windowController = p;
        }
    });

}

openDllWebView();