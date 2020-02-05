extern crate gl;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("A fantastic window!");

    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    let gl_context = windowed_context.context();

    println!(
        "Pixel format of the window's GL context: {:?}",
        windowed_context.get_pixel_format()
    );

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);
    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    el.run(move |event, _, control_flow| {
        println!("{:?}", event);
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
