extern crate glium;
extern crate ctrlc;
extern crate winit;
extern crate notify;

#[macro_use] extern crate objc;

mod handle;

use std::process::{Command, Child};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::thread::sleep;

use winit::os::macos::WindowExt;

use notify::{Watcher, RecursiveMode, watcher};
use notify::DebouncedEvent::{Write, Create};


////////////////////////////////////////////////////////////////////////////////

fn main() {
    use glium::glutin;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(400, 400)
        .with_title("Live")
       // .with_decorations(false)
        ;
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // Poke at the NSWindow to make it float
    let nswindow = display.gl_window().window().get_nswindow()
                   as *mut objc::runtime::Object;
    unsafe {
        msg_send![nswindow, setLevel:1];
        msg_send![nswindow, setHasShadow:0];
    };

    // Create a live-reloading handle to the library itself
    let mut handle = handle::Handle::new(
        "target/debug/liblive.dylib".to_string(),
        &display);

    // Configure a file watcher to rebuild if a file changes
    // (this is the equivalent to 'cargo watch')
    let (watch_tx, watch_rx) = channel();
    let mut watcher = watcher(watch_tx, Duration::from_millis(100))
        .expect("Couldn't create watcher");
    watcher.watch("src", RecursiveMode::Recursive)
        .expect("Couldn't start watching 'src'");
    watcher.watch(&handle.target, RecursiveMode::NonRecursive)
        .expect("Couldn't start watching target");

    // This command is time-consuming, so we move it into an Option
    // instead of waiting for it to finish before redrawing.
    let mut rebuild_cmd : Option<Child> = None;

    let mut counter = 0;
    while running.load(Ordering::SeqCst) {
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

        match watch_rx.try_recv() {
           Ok(event) => {
               match event {
                   Create(_) => {
                       handle.reload(&display);
                   }
                   Write(_) => {
                       rebuild_cmd.map(|mut c| { c.wait().expect("Failed to rebuild"); });
                       rebuild_cmd = Some(Command::new("cargo")
                           .args(&["build", "--lib"])
                           .spawn()
                           .expect("Failed to start 'cargo build --lib'"));
                   }
                   _ => (),
               }
           },
           Err(e) => match e {
               std::sync::mpsc::TryRecvError::Empty => (),
               _ => println!("Watch error: {:?}", e),
           },
        }

        // Manually cap the frame-rate at 60 FPS
        sleep(std::time::Duration::from_millis(1000/60));
    }
}
