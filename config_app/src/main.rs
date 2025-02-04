#[macro_use]
extern crate static_assertions;

use std::iter;
use eframe::{NativeOptions, Renderer};
use ui::launcher::Launcher;
use window::MainWindow;
use egui_wgpu::renderer::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use winit::event::Event::*;
use winit::event_loop::ControlFlow;

mod ui;
mod usb_hw;
mod window;

// IMPORTANT. On windows, only the i686-pc-windows-msvc target is supported (Due to limitations with J2534 and D-PDU!
#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
compile_error!("Windows can ONLY be built using the i686-pc-windows-msvc target!");

fn main() {
    env_logger::init();
    /*
    let event_loop = winit::event_loop::EventLoop::with_user_event();
    let window = winit::window::WindowBuilder::new()
        .with_decorations(true)
        .with_resizable(true)
        .with_title("Ultimate-NAG52 configuration utility")
        .with_inner_size(winit::dpi::PhysicalSize {
            width: 1280u32,
            height: 720u32
        })
        .build(&event_loop)
        .unwrap();
    */
    let mut app = window::MainWindow::new();
    app.add_new_page(Box::new(Launcher::new()));
    let mut native_options = NativeOptions::default();
    #[cfg(windows)]
    {
        native_options.renderer = Renderer::Wgpu;
    }
    eframe::run_native("Ultimate NAG52 config suite", native_options, Box::new(|cc| {
        Box::new(app)
    }));
}
