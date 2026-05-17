#![windows_subsystem = "windows"]

use std::env;
use std::num::NonZeroU32;
use std::path::Path;
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::Icon;
use tao::window::WindowBuilder; 

fn load_window_icon(path: &Path) -> Option<Icon> {
    if let Ok(img) = image::open(path) {
        let img_rgba = img.to_rgba8();
        let (width, height) = img_rgba.dimensions();
        let raw_data = img_rgba.into_raw();
        
        Icon::from_rgba(raw_data, width, height).ok()
    } else {
        None
    }
}

fn main() {
     

    let event_loop = EventLoop::new(); 
 
    let mut exe_path = env::current_exe().expect("Gagal mendapatkan lokasi file EXE"); 
    exe_path.pop();  
    let splash_path = exe_path.join("splash.png");
    let icon_path = exe_path.join("icon.png");

    let img = image::open(&splash_path)
        .expect("Gagal membuka file PNG")
        .to_rgba8();
    let (width, height) = img.dimensions(); 
    let window = WindowBuilder::new()
        .with_title("Splash Screen")
        .with_window_icon(load_window_icon(&icon_path))
        .with_inner_size(tao::dpi::PhysicalSize::new(width, height))
        .with_decorations(false) // Frameless
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    // Tembak redraw pertama kali saat aplikasi dibuka
    window.request_redraw();

    // 3. Jalankan Event Loop
    // Hanya memindahkan `window` dan `img` ke dalam closure
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            tao::event::Event::RedrawRequested(_) => {
                // KUNCI PERBAIKAN: Buat context dan surface DI DALAM loop saat event redraw dipanggil.
                // Dengan begini, lifetime context & surface hanya hidup di dalam scope ini
                // dan meminjam `&window` dengan aman.
                let context = softbuffer::Context::new(&window).unwrap();
                let mut surface = softbuffer::Surface::new(&context, &window).unwrap();

                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();

                // Salin pixel dari gambar PNG ke buffer
                for y in 0..height {
                    for x in 0..width {
                        let pixel = img.get_pixel(x, y);
                        let r = pixel[0] as u32;
                        let g = pixel[1] as u32;
                        let b = pixel[2] as u32;
                        let a = pixel[3] as u32;

                        let index = (y * width + x) as usize;

                        // Format Softbuffer: 0xAARRGGBB
                        buffer[index] = (a << 24) | (r << 16) | (g << 8) | b;
                    }
                }

                // Render ke layar
                buffer.present().unwrap();
            }
            tao::event::Event::WindowEvent { event, .. } => match event {
                tao::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            _ => {}
        }
    });
}
