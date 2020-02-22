pub mod assets;
pub mod graphics;
pub mod utils;

use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder, GlProfile, GlRequest,
};

use std::ffi::{CStr, CString};

use graphics::gl::shader::{Program, Shader};

use log::Level;
use utils::log::Logger;

fn main() {
    Logger::init_with_level(Level::Debug);
    log::info!("Game started");

    let window_config = assets::config::Window::load();

    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title(window_config.into_owned().title);

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

    let vertex_shader =
        Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();

    let fragment_shader =
        Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap();

    let program = Program::from_shaders(&[vertex_shader, fragment_shader]).unwrap();
    program.activate();

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
