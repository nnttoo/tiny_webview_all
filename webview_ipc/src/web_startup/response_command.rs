use std::{
    collections::HashMap,
    error::Error,
    process::Stdio,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, SyncSender},
    },
};

use tokio::{
    io::{AsyncReadExt, BufReader},
    process::Command,
};

pub struct CommandChild {
    pub cmd_id: u32,
    pub keep_alive: Option<SyncSender<bool>>,
    pub cmd_reader: Option<Receiver<Vec<u8>>>,
}

impl CommandChild {
    pub fn new() -> Self {
        Self {
            cmd_id: 0,
            keep_alive: None,
            cmd_reader: None,
        }
    }

    pub fn exec(&mut self, exepath: &str, args: Vec<String>) -> Result<(), &str> {
        let mut c = Command::new(exepath);
        c.args(args);

        c.stdout(Stdio::piped());
        c.stdin(Stdio::piped());

        let Ok(mut child) = c.spawn() else {
            return Err("cant spawn");
        };

        let Some(child_id) = child.id() else {
            return Err("cant get thread id");
        };

        self.cmd_id = child_id;

        let Some(stdout) = child.stdout.take() else {
            return Err("cant take child stdout");
        };

        let mut stdout_reader = BufReader::new(stdout);

        let (tx, rx) = mpsc::sync_channel::<bool>(1);

        // Haryanto 13/06/2026
        // create reciver for byte that whil use when
        // uiapi call  command_read
        let (tx_uiapi, rx_uiapi) = mpsc::sync_channel::<Vec<u8>>(3);
        self.cmd_reader = Some(rx_uiapi); // save the reciver

        self.keep_alive = Some(tx);

        tokio::spawn(async move {
            let mut buffer = [0u8; 1024];

            loop {
                let is_keep_alive = rx.try_recv().unwrap_or(true);
                if !is_keep_alive {
                    break;
                }

                match stdout_reader.read(&mut buffer).await {
                    Ok(0) => {
                        println!("Stream reached EOF (Process ended).");
                        break;
                    }
                    Ok(n) => {
                        let raw_bytes = &buffer[..n];
                        _ = tx_uiapi.send(raw_bytes.to_vec());
                    }
                    Err(e) => {
                        eprintln!("Error reading bytes: {}", e);
                        break;
                    }
                }
            }

            println!("thread END");
        });

        Ok(())
    }

    pub fn stop_thread(&self) {
        let Some(keep_alive) = &self.keep_alive else {
            return;
        };

        _ = keep_alive.send(false);
    }

    pub fn read_command(&mut self) -> Vec<u8> {
        let mut result = b"".to_vec();

        let Some(cmdreader) = &self.cmd_reader else {
            return result;
        };

        for mut r in cmdreader.try_iter() {
            result.append(&mut r); 
        }

        result
    }
}

type MutextChildArc = Arc<Mutex<CommandChild>>;
type BoxError = Box<dyn Error>;

#[derive(Clone)]
pub struct CommandManager {
    pub map_command: Arc<Mutex<HashMap<u32, MutextChildArc>>>,
}

impl CommandManager {
    pub fn new() -> Self {
        Self {
            map_command: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_command(&self, exepath: &str, args: Vec<String>) -> Result<u32, &str> {
        let Ok(mut lock) = self.map_command.try_lock() else {
            return Err("cant lock hashmap");
        };

        let mut mycmd = CommandChild::new();
        let Ok(_) = mycmd.exec(exepath, args) else {
            return Err("cant exec");
        };

        let cmd_id = mycmd.cmd_id;

        lock.insert(cmd_id, Arc::new(Mutex::new(mycmd)));

        println!("create_command calld {}", cmd_id);

        Ok(cmd_id)
    }

    fn get_child(&self, threadid: u32) -> Result<MutextChildArc, BoxError> {
        let Ok(hashmap_lock) = self.map_command.try_lock() else {
            return Err(Box::from("canot lock"));
        };

        let Some(child) = hashmap_lock.get(&threadid) else {
            return Err(Box::from("canot get child"));
        };

        Ok(child.clone())
    }

    pub fn stop_command(&self, proc_id: u32) -> Result<&str, BoxError> {
        let child = self.get_child(proc_id)?;

        let Ok(child_lock) = child.try_lock() else {
            return Err(Box::from("cant lock child"));
        };

        child_lock.stop_thread();
        Ok("thread stop success")
    }

    pub fn read_command(&self, thread_id: u32) -> Result<Vec<u8>, BoxError> {
        let child = self.get_child(thread_id)?;
        let Ok(mut child_lock) = child.try_lock() else {
            return Err(Box::from("cant lock child"));
        };

        let data = child_lock.read_command(); 
        Ok(data)
    }
}
