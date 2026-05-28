use std::sync::mpsc;

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    platform::windows::EventLoopBuilderExtWindows,
};
use tokio::task::JoinHandle;

use crate::app_ctx::{AppMyContext, CustomEvent};

pub fn create_event_loop() -> (AppMyContext, JoinHandle<()>) {
    let (tx, rx) = mpsc::channel::<AppMyContext>();

    let thread_handle = tokio::task::spawn_blocking(move || {
        let event_loop = EventLoopBuilder::<CustomEvent>::with_user_event()
            .with_any_thread(true)
            .build();

        let mut my_app_context = AppMyContext::new(event_loop.create_proxy());

        tx.send(my_app_context.clone()).unwrap();

        event_loop.run(move |event, elwt, control_flow| match event {
            Event::UserEvent(CustomEvent::Execute(myfun)) => {
                myfun(elwt);
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
                ..
            } => {
                println!("try to close window");
                if my_app_context.webview_remove(window_id) {
                    *control_flow = ControlFlow::Exit;
                } 
            }

            _ => (),
        });
    });

    let myapp_ctx = rx.recv().unwrap();

    (myapp_ctx, thread_handle)
}
