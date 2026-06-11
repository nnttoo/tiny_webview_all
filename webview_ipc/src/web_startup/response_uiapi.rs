use std::borrow::Cow;

use crate::web_startup::response::ResponseTools;

impl ResponseTools {
    fn get_body_str(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(self.req.body())
    }

    fn call_command(&self)->&[u8]{
        

        b""
    }

    fn print_log(&self) -> &[u8] {
        let bd = self.get_body_str();
        println!("webLog : {}", bd);

        b""
    }

    pub async fn ui_api(&self) -> &[u8] {
 

        let uipath_only = self.req_path.split_once("/uiapi/")
        .map(|(_,r)|r) 
        .unwrap_or(&self.req_path);

        match uipath_only {
            "printlog" => self.print_log(), 
            "command" => self.call_command(), 
            _ => b"uiapi not found",
        }
    }
}
