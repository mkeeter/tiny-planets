extern crate glium;
use glium::Surface;

struct State
{
}

impl State {
    fn new() -> State {
        State{}
    }
    fn draw(&self, counter :i32, frame : &mut glium::Frame) {
        frame.clear_color(0.0, 1.0, 1.0, 1.0);
    }
}

////////////////////////////////////////////////////////////////////////////////
// HERE BE DRAGONS
static mut state : Option<State> = None;

#[no_mangle]
pub extern "C" fn draw(counter : i32, frame : *mut glium::Frame) {
    unsafe {
        state.as_ref().unwrap().draw(counter, &mut *frame);
    }
}

#[no_mangle]
pub extern "C" fn init() {
    unsafe {
        state = Some(State::new());
    }
}

#[no_mangle]
pub extern "C" fn deinit() {
    unsafe {
        state.take();
    }
}
