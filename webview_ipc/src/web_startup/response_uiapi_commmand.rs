use crate::web_startup::{BoxError, response::ResponseTools};

impl ResponseTools {
    pub fn command_call(&self) -> Result<Vec<u8>, BoxError> {
        let bodystr = self.get_body_str().to_string();
        let w_json = &self.web_ctx.config.win_json;
        let hmap = &w_json.list_command;

        let Some(command) = hmap.get(&bodystr) else {
 
            return Err(Box::from("no command found"));
        };

        let cmd_manager = &self.web_ctx.command_mager;
        let cmdid = cmd_manager.create_command(
            &command.executable,
            (&command.args).clone(),
            self.web_ctx.clone(),
        )?;

        let response = format!("{}", cmdid).into_bytes();

        Ok(response)
    }

    pub fn command_stop(&self) -> Result<Vec<u8>, BoxError> {
        let bodystr = self.get_body_str().to_string();
        let Ok(thread_id) = bodystr.parse::<u32>() else {
            return Err(Box::from("error: body parse error"));
        };

        let Ok(_) = self.web_ctx.command_mager.stop_command(thread_id) else {
            return Err(Box::from("error: stop command error"));
        };

        Ok(b"ok".to_vec())
    }

    pub fn command_read(&self) -> Result<Vec<u8>, BoxError> {
        let bodystr = self.get_body_str().to_string();
        let Ok(thread_id) = bodystr.parse::<u32>() else {
            return Err(Box::from("error: body parse error"));
        };

        let Ok(data) = self.web_ctx.command_mager.read_command(thread_id) else { 
            return Err(Box::from("error: read command error"));
        };

        Ok(data)
    }
}
