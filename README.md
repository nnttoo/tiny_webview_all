# webview_ipc

A lightweight, cross-platform desktop application framework powered by [WRY](https://github.com/tauri-apps/wry) in Rust. Think of it as an ultra-minimalist alternative to Electron, designed to build modern desktop UIs using web technologies without the massive file size overhead.

### ⚠️ Project Status: In Development
This project is currently under active development and is **not yet ready for production**. However, it is progressing quickly, and a stable version is expected to be ready in the near future! Feel free to test it out, report bugs, or share your feedback.

## 🚀 Features

- **Ultra Lightweight:** Binary size is only around **3MB** on both Linux and Windows (compared to Electron's 100MB+).
- **Language Agnostic:** Built entirely around Inter-Process Communication (IPC). You can control your frontend from almost any backend language.
- **Powered by WRY:** Utilizes native webviews (WebView2 on Windows, WebKitGTK on Linux) via the robust Rust WRY library.
- **Fast & Efficient:** Low memory footprint and high performance.

## 🛠️ How it Works

Unlike traditional frameworks that tightly couple the frontend with a specific runtime (like Node.js in Electron), `webview_ipc` leverages a language-agnostic IPC layer. 

The core binary manages the native window and webview orchestration, while your backend application communicates with it over IPC channels to handle business logic, system API access, and data persistence.

## 📂 Examples

The repository includes ready-to-run examples demonstrating how to hook up your backend to the webview interface. 

Currently, implemented examples include:
- **Node.js**
- **PHP**

### Other Languages
Because the core architecture relies strictly on standard IPC protocols, **any language that supports IPC or standard I/O streams can be used as a backend** (such as Go, Python, C++, Rust, or Flutter). More official examples for other programming languages will be added in future updates. Contributions are always welcome!

## ⚙️ Requirements

- **Windows:** Windows 10/11 with WebView2 Runtime installed.
- **Linux:** `webkit2gtk` packages installed via your distribution's package manager.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.
 
 
 
## 🗺️ Roadmap & Progress

Here is the current development status of the project, ordered from the earliest milestones to upcoming features:

- ✅ Change project name to `webview_ipc`
- ✅ Create file server using IPC
- 🕧 Implement advanced event listeners
- 🕧 Add automated build pipelines for Windows, Linux, and macOS
- 🕧 Release stable v1.0.0 for production