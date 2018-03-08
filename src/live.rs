////////////////////////////////////////////////////////////////////////////////
// HERE BE DRAGONS
////////////////////////////////////////////////////////////////////////////////
extern crate glium;
mod draw;

static mut STATE : Option<draw::State> = None;

#[no_mangle]
pub extern "C" fn draw(counter : i32, frame : *mut glium::Frame) {
    unsafe {
        STATE.as_ref().unwrap().draw(counter, &mut *frame);
    }
}

#[no_mangle]
pub extern "C" fn init() {
    unsafe {
        STATE = Some(draw::State::new());
    }
}

#[no_mangle]
pub extern "C" fn deinit() {
    unsafe {
        STATE.take();
    }
}
