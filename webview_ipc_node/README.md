 # webview_ipc

`webview_ipc` is a lightweight Node.js wrapper for the Rust-based WebView IPC runtime.
It provides a native desktop webview experience with cross-platform IPC, window control, file dialogs, and a deployment helper for generating executable builds.

## 🚀 Why use `webview_ipc`

- **Ultra-lightweight**: Designed around a minimal Rust/Wry core instead of a full Electron runtime.
- **Cross-platform native webview**: Uses WebView2 on Windows and WebKitGTK on Linux through the Rust webview engine.
- **Language-agnostic IPC**: Frontend and backend communicate over a standard IPC channel, making integration simple and flexible.
- **Native window control**: Support for move, resize, minimize, maximize, close, and file/folder selection.
- **Deploy helper included**: Build and package native executables with `webview_ipc/builder`.

## ✅ Features

- `startWebIpcServer()` to serve local web content and receive IPC requests.
- `openWebview()` to open a native browser window with custom window settings.
- `selectFile()` / `selectFolder()` utilities for native file dialogs.
- `webview.close()`, `webview.move()`, `webview.resize()`, `webview.minimize()`, `webview.maximize()`.
- `deploy_exe()` builder API for creating Windows executable packages.

## 📦 Installation

```bash
npm install webview_ipc
```

## 🧩 Usage example

This example is adapted from `examples/webview_ipc/src/index.ts`.

```ts
import path from "node:path";
import { openWebview, selectFile, selectFolder, startWebIpcServer, WebControl, WebResponse } from "webview_ipc";

let ipcpath = process.env.IPCNAME ? process.env.IPCNAME : "err";

export interface FParam {
  cmd: string;
  params: any;
}

async function run() {
  let web: WebControl;
  let myipcpath = ipcpath + "mynodeipc";

  startWebIpcServer(
    myipcpath,
    path.join(__dirname, "../html"),
    async (req) => {
      if (req.path != "/controlwindow") {
        return null;
      }

      const simple_result = (msg: string) => ({
        body: Buffer.from(msg),
        content_type: "application/json",
      });

      const json = req.bodyJson<FParam>();
      if (!json?.cmd) return null;

      if (json.cmd === "close") {
        web.close();
        return simple_result("ok");
      }

      if (json.cmd === "move") {
        const arg = json.params as { top: number; left: number };
        web.move(arg.left, arg.top);
        return simple_result("ok");
      }

      if (json.cmd === "resize") {
        const arg = json.params as { width: number; height: number };
        web.resize(arg.width, arg.height);
        return simple_result("ok");
      }

      if (json.cmd === "minimize") {
        web.minimize(json.params as boolean);
        return simple_result("ok");
      }

      if (json.cmd === "maximize") {
        web.maximize(json.params as boolean);
        return simple_result("ok");
      }

      if (json.cmd === "select_file") {
        const file = await selectFile(json.params);
        return simple_result(file.message);
      }

      if (json.cmd === "select_folder") {
        const folder = await selectFolder(json.params);
        return simple_result(folder.message);
      }

      console.log(json.cmd, json.params);
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
      ipc_server: myipcpath,
    });
  } catch (error) {
    console.error("Node.js error", error);
  }
}

run();
```

## 🚀 Deploy example

Use the builder API from `examples/webview_ipc/src/builder/deploy.ts` to package a Windows executable.

```ts
import path from "node:path";
import { deploy_exe, Platform } from "webview_ipc/builder";

async function deploy() {
  deploy_exe({
    exeFilePath: path.join(__dirname, "../../dist/myApp.exe"),
    iconPath: path.join(__dirname, "../../icon.png"),
    startCommand: "dir",
    platform: Platform.Windows32,
  });
}

deploy();
```

## 🧪 Notes

- Ensure `WebView2` runtime is installed on Windows.
- For Linux, install `webkit2gtk` or the matching native webview dependency.
- This package is built around IPC and native window controls, so it is ideal for lightweight desktop apps that need a local web UI.

## 📄 License

`webview_ipc` is licensed under `ISC` as defined in `package.json`.
