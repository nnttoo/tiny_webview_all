use std::{
    env, fs,
    io::{BufRead, BufReader},
    os::windows::process::CommandExt,
    path::PathBuf,
    process::{Command, Stdio},
};

use crate::app_ctx::AppMyContextArc;

fn get_exe_folder() -> PathBuf {
    let mut path_def = PathBuf::new();
    let exe_path = env::current_exe();

    if let Ok(exe_path_ok) = exe_path {
        path_def = exe_path_ok;
        path_def.pop();
    };

    path_def
}

fn get_cmd_content() -> String {
    let cmd_path: PathBuf = {
        let file_path = "index_cmd";

        let cmd = get_exe_folder().join(file_path);
        match fs::exists(&cmd) {
            Ok(isexis) => {
                if isexis {
                    cmd
                } else {
                    file_path.into()
                }
            }
            _ => file_path.into(),
        }
    };

    let Ok(ctn) = fs::read_to_string(cmd_path) else {
        return "".to_string();
    };

    ctn
}

// pub fn execute_command(cmd_text: &str) -> Result<String, String> {
//     let mut command = if cfg!(target_os = "windows") {
//         let mut c = Command::new("cmd");
//         c.args(["/C", cmd_text]);
//         c
//     } else {
//         let mut c = Command::new("sh");
//         c.args(["-c", cmd_text]);
//         c
//     };

//     // Jalankan perintah
//     match command.output() {
//         Ok(output) => {
//             if output.status.success() {
//                 String::from_utf8(output.stdout).map_err(|e| format!(" UTF-8 invalid: {}", e))
//             } else {
//                 let err_msg = String::from_utf8_lossy(&output.stderr).to_string();
//                 Err(format!("Command error:\n{}", err_msg))
//             }
//         }
//         Err(e) => Err(format!("Failed shell execute: {}", e)),
//     }
// }

pub fn execute_command_live(cmd_text: &str, ipcname: String) -> Result<String, String> {
    let mut command = if cfg!(target_os = "windows") {
        let mut c = Command::new("cmd");
        c.args(["/C", cmd_text]);
        c.creation_flags(0x08000000);
        c
    } else {
        let mut c = Command::new("sh");
        c.args(["-c", cmd_text]);
        c
    };

    command.env("IPCNAME", ipcname);

    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command
        .spawn()
        .map_err(|e| format!("Failed to start command: {}", e))?;
    let stdout = child.stdout.take().ok_or("Failed to open stdout")?;
    let stderr = child.stderr.take().ok_or("Failed to open stderr")?;

    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);

    let stderr_handle = std::thread::spawn(move || {
        for line in stderr_reader.lines() {
            if let Ok(l) = line {
                eprintln!("{}", l);
            }
        }
    });
    let mut total_output = String::new();
    for line in stdout_reader.lines() {
        if let Ok(l) = line {
            println!("{}", l);
            total_output.push_str(&l);
            total_output.push('\n');
        }
    }
    let _ = stderr_handle.join();

    let status = child
        .wait()
        .map_err(|e| format!("Error waiting for process: {}", e))?;

    if status.success() {
        Ok(total_output)
    } else {
        Err(format!(
            "Command exited with companion error status: {}",
            status
        ))
    }
}

pub async fn exec_command(appctx: AppMyContextArc) {
    let ipcname = &appctx.ipc_name;

    println!("nama ipc nya {}", ipcname);

    let cmd_ctn = get_cmd_content();
    _ = execute_command_live(&cmd_ctn, ipcname.to_string());
    println!("Process END : {}", cmd_ctn);

    appctx.exit_window();
}
