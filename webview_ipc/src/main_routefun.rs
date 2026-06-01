use std::sync::OnceLock;

use serde::Deserialize;
use tao::event_loop::EventLoopWindowTarget;
use wry::dpi::{LogicalPosition, LogicalSize};

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
        println!("Parse Error {}", &msg.message);
        return cmd_msg("Parse Error");
    };

    ctx.call_ui_fun(move |_, ui_controller| {
        let Ok(uiitem) = ui_controller.get_window_byid(move_arg.win_id) else {
            println!("cannot get winows");

            return;
        };

        uiitem
            .window
            .set_outer_position(LogicalPosition::new(move_arg.left,move_arg.top));
    });

    cmd_msg("")
}

pub async fn web_resize(msg: CmdMessage) -> CmdMessage {
    let Some(ctx) = GLOBAL_CTX.get() else {
        return cmd_msg("ctx er");
    };

    #[derive(Deserialize)]
    struct ResizeArg {
        win_id: u32,
        height: u32,
        width: u32,
    }

    let Ok(resize_arg) = serde_json::from_str::<ResizeArg>(&msg.message) else {
        println!("json parse error {}", msg.message);
        return cmd_msg("parse error");
    };

    ctx.call_ui_fun(move |_, ui_controller| {
        let Ok(uiitem) = ui_controller.get_window_byid(resize_arg.win_id) else {

            println!("cant get window");
            return;
        };

        uiitem.window.set_inner_size(LogicalSize::new(resize_arg.width,resize_arg.height)); 
    });

    cmd_msg("ok")
}



pub async fn web_minimize(msg: CmdMessage) -> CmdMessage {
    let Some(ctx) = GLOBAL_CTX.get() else {
        return cmd_msg("ctx er");
    };

    #[derive(Deserialize)]
    struct Arg {
        win_id: u32, 
        minimize : bool
    }

    let Ok(arg) = serde_json::from_str::<Arg>(&msg.message) else {

        return cmd_msg("json parse error");
    };

    ctx.call_ui_fun(move |_, ui_controller| {
        let Ok(uiitem) = ui_controller.get_window_byid(arg.win_id) else {

            println!("cant get window");
            return;
        };

        uiitem.window.set_minimized(arg.minimize); 
    });


    cmd_msg("ok")
}