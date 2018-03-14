////////////////////////////////////////////////////////////////////////////////
// HERE BE DRAGONS
////////////////////////////////////////////////////////////////////////////////
extern crate glium;

mod draw;

use glium::Surface;

static mut STATE : Option<draw::draw::State> = None;

#[no_mangle]
pub extern "C" fn init(display : *const glium::Display) {
    unsafe {
        STATE = Some(draw::draw::State::new(&*display));
    }
}

#[no_mangle]
pub extern "C" fn draw(counter : i32, frame : *mut glium::Frame) {
    unsafe {
        STATE.as_ref().unwrap().draw(counter, &mut *frame);
    }
}

#[no_mangle]
pub extern "C" fn deinit() {
    unsafe {
        STATE.take();
    }
}
