use std::{ 
    sync::{
        Arc,
        atomic::{AtomicBool,  Ordering},
    },
};

use tao::{
    event_loop::{EventLoopProxy, EventLoopWindowTarget}, 
}; 

use crate::{ipc_server_handler::create_ipc_name, start_event_loop_ui::UiController};

type BoxedCommandUI =
    Box<dyn FnOnce(&EventLoopWindowTarget<CustomEvent>, &mut UiController) + Send + 'static>;
pub enum CustomEvent {
    ExecuteUI(BoxedCommandUI),
    Exit(),
}
//#[derive(Clone)]
pub struct AppMyContext {
    pub even_loop_poxy: EventLoopProxy<CustomEvent>,
    pub ipc_name: String,
    pub is_exit: Arc<AtomicBool>,
}

impl AppMyContext {
    pub fn new(event_loop: EventLoopProxy<CustomEvent>) -> AppMyContextArc {
        Arc::new(Self {
            even_loop_poxy: event_loop,
            ipc_name: create_ipc_name(),
            is_exit: Arc::new(AtomicBool::new(false)),
        })
    }
 

    pub fn webview_close(&self, wiid: u32) {  
        let command = Box::new( 
            move |_: &EventLoopWindowTarget<CustomEvent>, ui_controller: &mut UiController| { 
                ui_controller.remove_by_id(wiid); 
            },
        );

        _=self.even_loop_poxy.send_event(CustomEvent::ExecuteUI(command));
    }

    pub fn req_stop_all(&self, msg : &str) {
        println!("req_stop_all loop {}", msg);
        self.is_exit.store(true, Ordering::Relaxed);
    }

    pub fn command_is_finish(&self){ 
        println!("command is finish");
        _ = self.even_loop_poxy.send_event(CustomEvent::Exit());
    }
}

pub type AppMyContextArc = Arc<AppMyContext>;
