extern crate gl;
use glutin::{event::Event, event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, ContextBuilder};


pub unsafe fn make_board() {
    let events = EventLoop::new();
    let window = WindowBuilder::new();
    let gl = ContextBuilder::new().build_windowed(window, &events).unwrap().make_current().unwrap();
    gl::load_with(|sym| gl.get_proc_address(sym));
}