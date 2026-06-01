use crate::{app_ctx::AppMyContextArc, utils_tools::get_exe_folder}; 

 

use std::{
    fs,
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Child, Command, Stdio},
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
 
 
fn kill_native(pid: u32 ) {
    std::thread::spawn(move || { 
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            // Jalankan taskkill tanpa memunculkan jendela konsol baru
            let _ = Command::new("taskkill")
                .args(["/F", "/T", "/PID", &pid.to_string()])
                .creation_flags(0x08000000) 
                .status();
        }

        #[cfg(not(target_os = "windows"))]
        {
            // Di Linux, jalankan kill -9 dengan tanda minus pada PID untuk membunuh seluruh Process Group
            let pgid = format!("-{}", pid);
            let _ = Command::new("kill")
                .args(["-9", &pgid])
                .status();
        }
    });
}

fn waiting_to_kill(mut child:  Child, exit_signal: Arc<AtomicBool>) {
    loop {
        if exit_signal.load(Ordering::Relaxed) {
            println!("kill child");

            kill_native(child.id());
            
            break;
        }

        thread::sleep(Duration::from_millis(1000));
    }

    _=child.wait();
}

pub fn execute_command_live(
    cmd_text: &str,
    ipcname: String,
    exit_signal: Arc<AtomicBool>,
) -> Result<&str, String> {
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
 
    waiting_to_kill(child, exit_signal);

    let _ = stderr_handle.join();
    let _ = stderr_handle2.join();

    Ok("")
    
}

pub async fn exec_command(appctx: AppMyContextArc) {
    let ipcname = &appctx.ipc_name;

    println!("nama ipc nya {}", ipcname);

    let cmd_ctn = get_cmd_content();
    _ = execute_command_live(&cmd_ctn, ipcname.to_string(), appctx.is_exit.clone());
    println!("Process END : {}", cmd_ctn);

    appctx.command_is_finish();
}
