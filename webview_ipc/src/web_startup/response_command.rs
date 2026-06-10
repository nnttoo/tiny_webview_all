use std::{collections::HashMap, process::Stdio, sync::{Arc, Mutex}};

use anyhow::{Error, Result};
use tokio::{io::{AsyncReadExt, BufReader}, process::Command};

pub struct CommandChild {
    pub cmd_id: u32,
}

impl CommandChild {
    pub fn new() -> Self {
        Self { cmd_id: 0 }
    }

    pub fn exec(&mut self, exepath: &str, args: &[&str]) -> Result<()> {
        let mut c = Command::new("cmd");
        c.args(args);

        c.stdout(Stdio::piped());
        c.stdin(Stdio::piped());

        let mut child = c.spawn()?;

        self.cmd_id = child.id().ok_or(Error::msg("failed to get id"))?;

        let stdout = child
            .stdout
            .take()
            .ok_or(Error::msg("Failed to open stderr"))?;
        let stderr = child
            .stderr
            .take()
            .ok_or(Error::msg("Failed to open stderr"))?;

        let mut stdout_reader = BufReader::new(stdout);
        let  stderr_reader = BufReader::new(stderr);

        tokio::spawn(async move {
            let mut buffer = [0u8; 1024];

            loop {
                // Membaca data byte secara asinkron ke dalam buffer
                match stdout_reader.read(&mut buffer).await {
                    Ok(0) => {
                        println!("Stream reached EOF (Process ended).");
                        break;
                    }
                    Ok(n) => {
                        // 'n' adalah jumlah byte yang berhasil dibaca pada giliran ini
                        let raw_bytes = &buffer[..n];
                        println!("[BYTES RECEIVED] {:?}", raw_bytes);

                        // Jika Anda ingin mengubah bytes tersebut ke string secara manual:
                        if let Ok(text) = std::str::from_utf8(raw_bytes) {
                            print!("[AS TEXT] {}", text);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading bytes: {}", e);
                        break;
                    }
                }
            }
        });

        Ok(())
    }
}

type MutextChildArc  =  Arc<Mutex<CommandChild>>;

#[derive(Clone)]
pub struct CommandManager {
   pub map_command : Arc<Mutex<HashMap<u32,MutextChildArc>>>
}

impl CommandManager {
    pub fn new()->Self{
        Self {
            map_command : Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn create_command(&self, exepath: &str, args: &[&str])-> Result<()>{

        let mut lock = self.map_command.try_lock().map_err(|e|{Error::msg("")})?;
         
        let mut mycmd = CommandChild::new();
        mycmd.exec(exepath, args)?; 
        lock.insert(mycmd.cmd_id, Arc::new(Mutex::new(mycmd))); 
        

        Ok(())
    }
}
