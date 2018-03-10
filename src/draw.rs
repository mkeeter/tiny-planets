extern crate glium;
use glium::Surface;

pub struct State
{
}

impl State {
    pub fn new(_width : u32, _height : u32) -> State {
        State{}
    }
    pub fn draw(&self, _counter : i32, frame : &mut glium::Frame) {
        frame.clear_color(0.4, 0.4, 0.0, 1.0);
    }
}
