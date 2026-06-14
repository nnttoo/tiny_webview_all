use std::borrow::Cow;

use crate::web_startup::{BoxError, response::ResponseTools};

impl ResponseTools {
    pub fn get_body_str(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(self.req.body())
    } 

    fn print_log(&self) -> Result<Vec<u8>, BoxError> {
        let bd = self.get_body_str();
        println!("webLog : {}", bd);

        Ok(b"".to_vec())
    }

    pub async fn ui_api(&self) -> Result<Vec<u8>, BoxError>{ 

        let uipath_only = self.req_path.split_once("/uiapi/")
        .map(|(_,r)|r) 
        .unwrap_or(&self.req_path);

        match uipath_only {
            "printlog" => self.print_log(), 
            "command" => self.command_call(), 
            "command_stop" => self.command_stop(), 
            "command_read" => self.command_read(), 
            _ => Err(Box::from("ui api not found")),
        }
    }
}
