use std::sync::OnceLock;

use serde::Deserialize;
use tao::event_loop::EventLoopWindowTarget;
use wry::dpi::LogicalPosition;

use crate::{
    app_ctx::AppMyContextArc, ipc_server::CmdMessage, open_web, start_event_loop::CustomEvent,
    start_event_loop_ui::UiController,
};

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

    ctx.call_ui_fun(
        move |_: &EventLoopWindowTarget<CustomEvent>, ui_controller: &mut UiController| {
            ui_controller.remove_by_id(wid);
        },
    );

    cmd_msg("close web ok")
}

pub async fn web_move(msg: CmdMessage) -> CmdMessage {
    let Some(ctx) = GLOBAL_CTX.get() else {
        return cmd_msg("ctx er");
    };


    #[derive(Deserialize)]
    struct MoveArg {
        win_id: u32,
        left: u32,
        top: u32,
    }

    let Ok(move_arg) = serde_json::from_str::<MoveArg>(&msg.message) else {
        
        println!("Parse Error {}",&msg.message);
        return cmd_msg("Parse Error");
    };

    ctx.call_ui_fun(move |_, ui_controller| {
        let Ok(_) = ui_controller.get_window_byid(move_arg.win_id, |w| {
            let left = move_arg.left;
            let top = move_arg.top;

            w.window.set_outer_position(LogicalPosition::new(left, top));
        }) else {
            println!("error move window");
            return;
        };
    });
 

    cmd_msg("")
}
