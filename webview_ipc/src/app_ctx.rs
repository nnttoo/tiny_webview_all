use std::{sync::{
    Arc, RwLock, atomic::{AtomicBool, Ordering}
}, thread::{self, ThreadId}};

use tao::event_loop::{EventLoopProxy, EventLoopWindowTarget};

use crate::{start_event_loop::CustomEvent, start_event_loop_ui::UiController};

//#[derive(Clone)]
pub struct AppMyContext {
    pub even_loop_poxy: EventLoopProxy<CustomEvent>, 
    pub is_exit: Arc<AtomicBool>,  
    pub thread_id : ThreadId
}

impl AppMyContext {
    pub fn new(event_loop: EventLoopProxy<CustomEvent>, ui_threadid : ThreadId) -> AppMyContextArc {
        Arc::new(Self {
            even_loop_poxy: event_loop,
            thread_id : ui_threadid,
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

    pub fn is_ui_thread(&self)->bool{
        let threadid = thread::current().id();
 

        println!("uithread {:?}, current thread : {:?}" , self.thread_id, threadid);
        return threadid == self.thread_id;
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
