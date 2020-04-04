use std::ffi::CStr;
use std::path::Path;

use glutin::{
    ContextBuilder,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    GlProfile, GlRequest, window::WindowBuilder,
};
use log::Level;

use utils::log::Logger;

use crate::assets::shader_asset::load_shader_assets;
use crate::assets::window_config::load_window_config;
use crate::graphics::gl::shader::shader::compile_shaders;

pub mod assets;
pub mod graphics;
pub mod utils;

fn main() {
    Logger::init_with_level(Level::Debug);
    log::info!("Game started");

    let shader_paths = vec![Path::new("assets/shaders/triangle.shader")];
    log::debug!("Shader paths: {:?}", shader_paths);

    let (shader_id_lookup_map, shader_assets) = load_shader_assets(&shader_paths);
    log::debug!("Shader id lookup map: {:?}", shader_id_lookup_map);
    log::debug!("Shader assets: {:?}", shader_assets);

    let window_config = load_window_config(Path::new("assets/window.ron"));
    log::debug!("Window config: {:?}", window_config);

    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title(window_config.title);

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

    let shaders = compile_shaders(&shader_assets);
    log::debug!("Compiled shaders: {:?}", shaders);

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
                // Reload modified shaders:
                // # Watch for changes to meta and source files
                // # Figure out which meta files to reload
                //   Associate files with asset ids
                // # Load modified shader assets
                // # Compile modified shaders
                // # Merge reloaded shaders into existing ones ("commit reload?")
                //   Replace reloaded shaders in the vector
                // # Figure out which shader programs need to relink
                //   Associate shader ids with shader program ids
                // # Link modified shader programs
                // # Merge reloaded shader programs into existing ones ("commit reload?")
                //   Replace reloaded shader programs in the vector

                // Reload modified shader programs:
                // # Watch for changes to shader program meta files
                // # Load modified shader program assets
                // # Link modified shader programs
                // # Merge reloaded shader programs into existing ones ("commit reload?")

                // Reload modified sprite(?):
                // ?

                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
