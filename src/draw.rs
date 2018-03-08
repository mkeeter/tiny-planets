extern crate glium;
use glium::Surface;

pub struct State
{
}

impl State {
    pub fn new(width : u32, height : u32) -> State {
        State{}
    }
    pub fn draw(&self, _counter :i32, frame : &mut glium::Frame) {
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
    }
}
