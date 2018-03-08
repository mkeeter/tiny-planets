extern crate glium;
extern crate ctrlc;

mod handle;

use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    use glium::glutin;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut counter = 0;

    // Run 'cargo watch' to rebuild libraries in the background
    let mut child = Command::new("cargo")
                           .args(&["watch", "-q", "-c", "-x", "build --lib"])
                           .spawn()
                           .expect("Failed to start 'cargo watch'");

    let mut handle = handle::Handle::new(
        "target/debug/liblive.dylib".to_string());
    while running.load(Ordering::SeqCst) {
        handle.check();

        handle.draw(counter, display.draw());
        counter += 1;

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => running.store(false, Ordering::SeqCst),
                    _ => (),
                },
                _ => (),
            }
        });
    }

    child.kill().expect("Couldn't kill 'cargo watch' subprocess");
}
