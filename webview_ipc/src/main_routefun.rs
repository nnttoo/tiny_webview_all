use std::sync::OnceLock;

use crate::{app_ctx::AppMyContextArc, ipc_server::CmdMessage, open_web};

static GLOBAL_CTX: OnceLock<AppMyContextArc> = OnceLock::new();

pub fn set_appctx_static(appctx: AppMyContextArc) {
    _ = GLOBAL_CTX.set(appctx);
}

fn cmd_msg(msg: &str) -> CmdMessage {
    CmdMessage::new("", msg)
}

pub async fn web_open(msg: CmdMessage) -> CmdMessage {
    let winid = (|| {
        let Some(ctx) = GLOBAL_CTX.get() else {
            return 0;
        };
        let Ok(wiid) = open_web::open_web(ctx.clone(), msg.message) else {
            return 0;
        };

        wiid
    })();

    cmd_msg(&format!("{}", winid))
}

pub async fn web_close(msg: CmdMessage) -> CmdMessage {
    let Some(ctx) = GLOBAL_CTX.get() else {
        return cmd_msg("app ctx not found");
    };

    let Ok(wid) = msg.message.parse::<u32>() else {
        return cmd_msg("win id parse error");
    };

    ctx.webview_close(wid);
    cmd_msg("close web ok")
}

pub async fn web_move(msg: CmdMessage) -> CmdMessage {
    let Some(ctx) = GLOBAL_CTX.get() else {
        return cmd_msg("ctx er");
    };
 

    cmd_msg("")
}
