use std::sync::{
    Arc, RwLock, atomic::{AtomicBool, Ordering}
};

use tao::event_loop::{EventLoopProxy, EventLoopWindowTarget};

use crate::{start_event_loop::CustomEvent, start_event_loop_ui::UiController};

//#[derive(Clone)]
pub struct AppMyContext {
    pub even_loop_poxy: EventLoopProxy<CustomEvent>, 
    pub is_exit: Arc<AtomicBool>, 
}

impl AppMyContext {
    pub fn new(event_loop: EventLoopProxy<CustomEvent>) -> AppMyContextArc {
        Arc::new(Self {
            even_loop_poxy: event_loop,
            //ipc_name: create_ipc_name(),
            is_exit: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn req_stop_all(&self, msg: &str) {
        println!("req_stop_all loop {}", msg);
        self.is_exit.store(true, Ordering::Relaxed);
    }

    pub fn command_is_finish(&self) {
        println!("command is finish");
        _ = self.even_loop_poxy.send_event(CustomEvent::Exit());
    }

    pub fn call_ui_fun<F>(&self, cb: F)
    where
        F: FnOnce(&EventLoopWindowTarget<CustomEvent>, &mut UiController) + Send + 'static,
    { 

        _ = self
            .even_loop_poxy
            .send_event(CustomEvent::ExecuteUI(Box::new(cb)));
    }
}

pub type AppMyContextArc = Arc<AppMyContext>;
