use std::borrow::Cow;

use crate::web_startup::{response::ResponseTools};

impl ResponseTools {
    pub fn get_body_str(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(self.req.body())
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
