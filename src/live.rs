////////////////////////////////////////////////////////////////////////////////
// HERE BE DRAGONS
////////////////////////////////////////////////////////////////////////////////
extern crate glium;
mod draw;

use glium::Surface;

static mut STATE : Option<draw::State> = None;

#[no_mangle]
pub extern "C" fn draw(counter : i32, frame : *mut glium::Frame) {
    unsafe {
        if STATE.is_none() {
            let (w,h) = (*frame).get_dimensions();
            STATE = Some(draw::State::new(w, h));
        }
        STATE.as_ref().unwrap().draw(counter, &mut *frame);
    }
}

#[no_mangle]
pub extern "C" fn deinit() {
    unsafe {
        STATE.take();
    }
}
