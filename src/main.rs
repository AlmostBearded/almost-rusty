use std::ffi::CStr;
use std::path::Path;

use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder, GlProfile, GlRequest,
};
use log::Level;

use assets::ShaderAssetManager;
use assets::{Asset, Store};
use graphics::gl::shader::Program;
use utils::log::Logger;

pub mod assets;
pub mod graphics;
pub mod utils;

fn main() {
    Logger::init_with_level(Level::Debug);
    log::info!("Game started");

    let shaderAssetManager = ShaderAssetManager {
        meta_paths: vec!["assets/shaders/triangle.frag.meta", "assets/shaders/triangle.vert.meta"]
    }

    // let window_config = database::config::Window::load();

    let el = EventLoop::new();
    // let wb = WindowBuilder::new().with_title(window_config.into_owned().title);
    let wb = WindowBuilder::new().with_title("TITLE");

    let windowed_context = ContextBuilder::new()
        .with_gl(GlRequest::Latest)
        .with_gl_profile(GlProfile::Core)
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    let gl_context = windowed_context.context();

    log::info!(
        "Pixel format of the window's GL context: {:?}",
        windowed_context.get_pixel_format()
    );

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);
    let version = unsafe {
        String::from_utf8(
            CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
                .to_bytes()
                .to_vec(),
        )
        .unwrap()
    };
    log::info!("OpenGL version: {}", version);

    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // let vertex_shader = assets::database::shaders::TRIANGLE_VERT.load().unwrap();
    //
    // let fragment_shader = assets::database::shaders::TRIANGLE_FRAG.load().unwrap();
    //
    // let program = Program::from_shaders(&[vertex_shader, fragment_shader]).unwrap();
    // program.activate();

    el.run(move |event, _, control_flow| {
        log::trace!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
