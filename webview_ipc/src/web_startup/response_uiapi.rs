use std::borrow::Cow;

use crate::web_startup::{response::ResponseTools, response_command::STOP_MSG_DELIMITER};

impl ResponseTools {
    fn get_body_str(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(self.req.body())
    }

    fn command_call(&self)->Vec<u8>{
        let bodystr  = self.get_body_str().to_string();
        let w_json = &self.web_ctx.config.win_json;  
        let hmap  = &w_json.list_command;

        let Some(command) = hmap.get(&bodystr) else {

            return b"no command found".to_vec();
        };

        let cmd_manager = &self.web_ctx.command_mager; 
        match cmd_manager.create_command(&command.executable, (&command.args).clone(),self.web_ctx.clone()){
            Err(eer)=>{ 
                 format!("error command {}", eer).into_bytes() 
            },
            Ok(cmdid)=>{
                format!("{}", cmdid).into_bytes()
            },
            _=>{
                b"unkown error".to_vec()
            }
        }
 
    }

    fn command_stop(&self)-> Vec<u8>{
        let bodystr  = self.get_body_str().to_string();
        let Ok(thread_id) = bodystr.parse::<u32>() else {
            return b"error parse thread id".to_vec();
        };

        let Ok(_)= self.web_ctx.command_mager.stop_command(thread_id) else {
            return b"cant stop command".to_vec();
        };

        b"ok".to_vec()
    }

    fn command_read(&self)->Vec<u8>{
        let bodystr  = self.get_body_str().to_string();
        let Ok(thread_id) = bodystr.parse::<u32>() else {
            return b"error parse thread id".to_vec();
        };

        let Ok(data) = self.web_ctx.command_mager.read_command(thread_id) else {
            return  STOP_MSG_DELIMITER.to_vec();
        };
        
        data
    }

    fn print_log(&self) -> Vec<u8> {
        let bd = self.get_body_str();
        println!("webLog : {}", bd);

        b"".to_vec()
    }

    pub async fn ui_api(&self) -> Vec<u8> {
 

        let uipath_only = self.req_path.split_once("/uiapi/")
        .map(|(_,r)|r) 
        .unwrap_or(&self.req_path);

        match uipath_only {
            "printlog" => self.print_log(), 
            "command" => self.command_call(), 
            "command_stop" => self.command_stop(), 
            "command_read" => self.command_read(), 
            _ => b"uiapi not found".to_vec(),
        }
    }
}
