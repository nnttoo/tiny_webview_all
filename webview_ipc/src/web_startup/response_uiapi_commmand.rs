use crate::web_startup::{response::ResponseTools, response_command::STOP_MSG_DELIMITER};

impl ResponseTools {
    pub fn command_call(&self) -> Vec<u8> {
        let bodystr = self.get_body_str().to_string();
        let w_json = &self.web_ctx.config.win_json;
        let hmap = &w_json.list_command;

        let Some(command) = hmap.get(&bodystr) else {
            return b"no command found".to_vec();
        };

        let cmd_manager = &self.web_ctx.command_mager;
        match cmd_manager.create_command(
            &command.executable,
            (&command.args).clone(),
            self.web_ctx.clone(),
        ) {
            Err(eer) => format!("error command {}", eer).into_bytes(),
            Ok(cmdid) => format!("{}", cmdid).into_bytes(),
        }
    }

    pub fn command_stop(&self) -> Vec<u8> {
        let bodystr = self.get_body_str().to_string();
        let Ok(thread_id) = bodystr.parse::<u32>() else {
            return b"error parse thread id".to_vec();
        };

        let Ok(_) = self.web_ctx.command_mager.stop_command(thread_id) else {
            return b"cant stop command".to_vec();
        };

        b"ok".to_vec()
    }

    pub fn command_read(&self) -> Vec<u8> {
        let bodystr = self.get_body_str().to_string();
        let Ok(thread_id) = bodystr.parse::<u32>() else {
            return b"error parse thread id".to_vec();
        };

        let Ok(data) = self.web_ctx.command_mager.read_command(thread_id) else {
            return STOP_MSG_DELIMITER.to_vec();
        };

        data
    }
}
