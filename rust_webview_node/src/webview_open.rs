
use tao::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct WebViewConfig {
    pub url: String,
    pub wclassname: String,
    pub custom_protocol: String,
    pub title: String,

    pub on_custom_protocol: Box<dyn Fn(&str)>,

    pub width: i32,
    pub height: i32,

    pub is_kiosk: bool,
    pub is_maximize: bool,
    pub is_debug: bool,
}

pub fn open_webview(webviewcon: &WebViewConfig) {

    let event_loop = EventLoop::new();
    let mut _builder = WindowBuilder::new() 
        .with_title(webviewcon.title.as_str())
        .with_inner_size(tao::dpi::LogicalSize::new(webviewcon.width, webviewcon.height));
      

    if webviewcon.is_kiosk {
        _builder = _builder.with_decorations(false);
    };

    if webviewcon.is_maximize {
        _builder = _builder.with_maximized(true);
    }
    


    println!("ini title {}", webviewcon.title);
    println!("ini wclassname {}", webviewcon.wclassname);
    println!("ini url {}", webviewcon.url);
    println!("ini width {}", webviewcon.width);
    println!("ini is_kiosk {}", webviewcon.is_kiosk);

    let _mywindow =  _builder.build(&event_loop).unwrap();

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
