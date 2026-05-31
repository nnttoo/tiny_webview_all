use crate::{app_ctx::AppMyContextArc, utils_tools::get_exe_folder};
use std::{
    fs,
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

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

fn waiting_to_kill(pid: u32, exit_signal: Arc<AtomicBool>) {
    loop {
        if exit_signal.load(Ordering::Relaxed) {
            #[cfg(target_os = "windows")]
            {
                use std::os::windows::process::CommandExt;
                let _ = Command::new("taskkill")
                    .args(["/F", "/T", "/PID", &pid.to_string()])
                    .creation_flags(0x08000000)
                    .spawn();
            }

            #[cfg(not(target_os = "windows"))]
            {
                // Di Linux, kirim SIGKILL (9) ke seluruh Process Group (-PID)
                // Nilai negatif dari PID memberi tahu OS untuk membunuh grup tersebut
                unsafe {
                    libc::kill(-(pid as libc::pid_t), libc::SIGKILL);
                }
            }
            break;
        }

        thread::sleep(Duration::from_millis(1000));
    }
}

pub fn execute_command_live(
    cmd_text: &str,
    ipcname: String,
    exit_signal: Arc<AtomicBool>,
) -> Result<String, String> {
    let mut command = if cfg!(target_os = "windows") {
        let mut c = Command::new("cmd");
        c.args(["/C", cmd_text]);
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            // 0x08000000 adalah CREATE_NO_WINDOW
            // Pastikan flags ini diatur tepat sebelum group_spawn()
            c.creation_flags(0x08000000);
        }
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

    let stderr_handle2 = std::thread::spawn(move || {
        for line in stdout_reader.lines() {
            if let Ok(l) = line {
                println!("[child] {}", l);
            }
        }
    });

    let pid = child.id();
    waiting_to_kill(pid, exit_signal);

    let _ = stderr_handle.join();
    let _ = stderr_handle2.join();

    let status = child
        .wait()
        .map_err(|e| format!("Error waiting for process: {}", e))?;

    if status.success() {
        Ok("".to_string())
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
    _ = execute_command_live(&cmd_ctn, ipcname.to_string(), appctx.is_exit.clone());
    println!("Process END : {}", cmd_ctn);

    appctx.command_is_finish();
}
