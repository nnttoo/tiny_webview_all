
use tao::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder; 
fn main() -> wry::Result<()> { 
    let event_loop = EventLoop::new();
 
    let window = WindowBuilder::new()
        .with_title("Wry Native Window")
        .with_inner_size(tao::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .unwrap(); 
    let _webview = WebViewBuilder::new()   
        .with_custom_protocol("wry".into(), |_id, _request| { 
            
            let host = _request.uri().path(); 
            let mybody = String::from("") +  host;
            let mybodyarg = mybody.into_bytes();

            wry::http::Response::builder()
                .header("Content-Type", "text/html")
                .body(std::borrow::Cow::Owned(mybodyarg))
                .unwrap()
        })
        .with_url("wry://mylocal/index.html")
        .build(&window)?;
 
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            tao::event::Event::WindowEvent {
                event: tao::event::WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}