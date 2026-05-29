#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use image::{ImageBuffer, Rgba};
use std::env;
use std::num::NonZeroU32;
use std::path::{Path, PathBuf};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::Icon;
use tao::window::WindowBuilder;

fn load_window_icon(exe_path: &Path) -> Option<Icon> {
    let path = exe_path.join("icon.png");

    if let Ok(img) = image::open(path) {
        let img_rgba = img.to_rgba8();
        let (width, height) = img_rgba.dimensions();
        let raw_data = img_rgba.into_raw();

        Icon::from_rgba(raw_data, width, height).ok()
    } else {
        None
    }
}

fn get_exe_folder() -> PathBuf {
    let mut path_def = PathBuf::new();
    let exe_path = env::current_exe();

    if let Ok(exe_path_ok) = exe_path {
        path_def = exe_path_ok;
        path_def.pop();
    };

    path_def
}

fn get_img_splash(path: &Path) -> Option<(ImageBuffer<Rgba<u8>, Vec<u8>>, u32, u32)> {
    let splash_path = path.join("splash.png");

    let img = image::open(&splash_path);
    let Ok(img_unwrap) = img else {
        return None;
    };

    let img_rgba = img_unwrap.to_rgba8();
    let (width, height) = img_rgba.dimensions();
    Some((img_rgba, width, height))
}

fn main() {
    let event_loop = EventLoop::new();

    let exe_path = get_exe_folder();

    

    let mut splash_img  = None;
    let mut splash_width: u32 = 300;
    let mut splash_height: u32 = 300;

    if let Some((img, w, h)) = get_img_splash(&exe_path) {
        splash_width = w;
        splash_height = h;
        splash_img = Some(img);
    }

    let window = WindowBuilder::new()
        .with_title("Splash Screen")
        .with_window_icon(load_window_icon(&exe_path))
        .with_inner_size(tao::dpi::PhysicalSize::new(splash_width, splash_height))
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
                let context = softbuffer::Context::new(&window).unwrap();
                let mut surface = softbuffer::Surface::new(&context, &window).unwrap();

                surface
                    .resize(
                        NonZeroU32::new(splash_width).unwrap(),
                        NonZeroU32::new(splash_height).unwrap(),
                    )
                    .unwrap();

                if let Some(img) = &splash_img {
                    let mut buffer = surface.buffer_mut().unwrap();
                    for y in 0..splash_width {
                        for x in 0..splash_height {
                            let pixel = img.get_pixel(x, y);
                            let r = pixel[0] as u32;
                            let g = pixel[1] as u32;
                            let b = pixel[2] as u32;
                            let a = pixel[3] as u32;

                            let index = (y * splash_width + x) as usize;

                            // Format Softbuffer: 0xAARRGGBB
                            buffer[index] = (a << 24) | (r << 16) | (g << 8) | b;
                        }
                    }

                    buffer.present().unwrap();
                }
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
