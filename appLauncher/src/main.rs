#![windows_subsystem = "windows"]
use std::ffi::c_void;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
}; 
use windows::{
    Win32::Foundation::HWND,
    core::{HSTRING, PCWSTR},
};
// 1. Pindahkan GetClientRect ke WindowsAndMessaging
use windows::Win32::Graphics::Gdi::{BeginPaint, EndPaint, PAINTSTRUCT};
use windows::Win32::Graphics::GdiPlus::{
    GdipCreateFromHDC, GdipDeleteGraphics, GdipDrawImageRectI, GdipLoadImageFromFile,
    GdiplusShutdown, GdiplusStartup, GdiplusStartupInput, GpGraphics, GpImage, Status,
};
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;

fn main() {
    // 1. Inisialisasi GDI+
    let mut gdiplus_token: usize = 0;
    let startup_input = GdiplusStartupInput {
        GdiplusVersion: 1,
        ..Default::default()
    };
    unsafe {
        let _ = GdiplusStartup(&mut gdiplus_token, &startup_input, std::ptr::null_mut());
    }

    // 2. Buat Event Loop dan Window Frameless dengan TAO
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("TAO Frameless PNG")
        .with_decorations(false)
        .with_inner_size(tao::dpi::LogicalSize::new(400.0, 400.0))
        .with_transparent(true)
        .build(&event_loop)
        .unwrap();

    use tao::platform::windows::WindowExtWindows;
    let hwnd = HWND(window.hwnd() as *mut c_void);

    let mut exe_path = std::env::current_exe().expect("Gagal mendapatkan path EXE");
    exe_path.pop();
    let img_path = exe_path.join("splash.png");
    let path_hstring = HSTRING::from(img_path.to_string_lossy().as_ref());
    let path_pcwstr: PCWSTR = windows::core::PCWSTR(path_hstring.as_ptr());
    let mut image_ptr: *mut GpImage = std::ptr::null_mut();
    unsafe {
        let _ = GdipLoadImageFromFile(path_pcwstr, &mut image_ptr);
    }

    // 4. Masuk ke Event Loop Aplikasi
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    unsafe {
                        GdiplusShutdown(gdiplus_token);
                    }
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            },

            Event::RedrawRequested(_) => {
                unsafe {
                    let mut ps = PAINTSTRUCT::default();
                    let hdc = BeginPaint(hwnd, &mut ps);

                    if !hdc.is_invalid() && !image_ptr.is_null() {
                        let mut graphics: *mut GpGraphics = std::ptr::null_mut();

                        // 2. PERBAIKAN: Status sukses GDI+ diwakili oleh nilai 0 / Status(0)
                        if GdipCreateFromHDC(hdc, &mut graphics) == Status(0) {
                            let mut rc = windows::Win32::Foundation::RECT::default();
                            let _ = GetClientRect(hwnd, &mut rc);

                            let width = rc.right - rc.left;
                            let height = rc.bottom - rc.top;

                            let _ = GdipDrawImageRectI(
                                graphics, image_ptr, rc.left, rc.top, width, height,
                            );

                            let _ = GdipDeleteGraphics(graphics);
                        }
                    }
                    EndPaint(hwnd, &ps);
                }
            }

            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}
