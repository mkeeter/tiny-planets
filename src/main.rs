extern crate glium;
extern crate libloading;
extern crate ctrlc;

use std::fs;
use std::time;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use libloading::{Library, Symbol};

struct Handle {
    target : String,
    lib : Option<Library>,
    path : Option<String>,
    modified : u64,
}

impl Handle {
    fn new(target : String) -> Handle {
        let mut out = Handle { target : target,
                               lib : None,
                               path : None,
                               modified : 0,
        };
        out.check();
        out
    }

    fn deinit(&mut self) {
        self.lib.as_ref().map(|lib| {
            unsafe {
                let deinit : Symbol<unsafe extern "C" fn()> =
                    lib.get(b"deinit\0").unwrap();
                deinit();
            }
        });
    }

    fn draw(&mut self, counter : i32, mut frame : glium::Frame) {
        self.lib.as_ref().map(|lib| {
            unsafe {
                let draw : Symbol<unsafe extern "C" fn(i32, *mut glium::Frame)> =
                    lib.get(b"draw\0").unwrap();
                draw(counter, &mut frame);
            }
        });
        frame.finish().unwrap();
    }

    fn timestamp(&self, t : time::SystemTime) -> u64 {
        t.duration_since(time::UNIX_EPOCH).unwrap().as_secs()
    }

    fn check(&mut self) {
        fs::metadata(&self.target).and_then(
            |m| {
                m.modified().map(
                |t| {
                let t = self.timestamp(t);
                if t > self.modified {
                    self.reload(t);
                    self.modified = t;
                }
            })
        }).expect("Could not get file metadata");
    }

    fn reload(&mut self, t : u64) {
        self.deinit();
        self.path.as_ref().map(fs::remove_file);

        let new_path = format!("{}.{}", self.target, t);
        Command::new("install_name_tool")
            .args(&["-id", &new_path, &self.target])
            .spawn()
            .expect("Failed to call 'install_name_tool'")
            .wait()
            .expect("Failed to rename library with 'install_name_tool'");
        fs::copy(&self.target, &new_path).expect("Could not copy library");

        self.lib = Library::new(&new_path).ok();
        self.path = Some(new_path);
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        self.path.as_ref().map(fs::remove_file);
    }
}

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

    let mut handle = Handle::new("target/debug/liblive.dylib".to_string());
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
