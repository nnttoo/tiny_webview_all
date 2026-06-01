use std::sync::mpsc;

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder, EventLoopWindowTarget},
};
use tokio::task::JoinHandle;

use crate::{
    app_ctx::{AppMyContext, AppMyContextArc},
    start_event_loop_ui::UiController,
};
 
#[cfg(target_os = "windows")]
use tao::platform::windows::EventLoopBuilderExtWindows;

pub type BoxedCommandUI =
    Box<dyn FnOnce(&EventLoopWindowTarget<CustomEvent>, &mut UiController) + Send + 'static>;
pub enum CustomEvent {
    ExecuteUI(BoxedCommandUI),
    Exit(),
}



pub fn create_event_loop() -> (AppMyContextArc, JoinHandle<()>) {
    let (tx, rx) = mpsc::channel::<AppMyContextArc>();

    let thread_handle = tokio::task::spawn_blocking(move || {
        let mut builder = EventLoopBuilder::<CustomEvent>::with_user_event();

        #[cfg(target_os = "windows")]
        {
            builder.with_any_thread(true);
        }

        let event_loop = builder.build();

        let my_app_context = AppMyContext::new(event_loop.create_proxy());
        _ = tx.send(my_app_context.clone());

        let mut ui_controller = UiController::new(my_app_context.clone());

        event_loop.run(move |event, elwt, control_flow| match event {
            Event::UserEvent(CustomEvent::ExecuteUI(myfun)) => {
                myfun(elwt, &mut ui_controller);
            }

            Event::UserEvent(CustomEvent::Exit()) => {
                *control_flow = ControlFlow::Exit;
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
                ..
            } => {
                ui_controller.remove(window_id);
            }

            _ => (),
        });
    });

    let myapp_ctx = rx.recv().unwrap();

    (myapp_ctx, thread_handle)
}
