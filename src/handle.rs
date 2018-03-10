extern crate glium;
extern crate libloading;

use std::time;
use std::fs;
use std::process::Command;
use self::libloading::{Library, Symbol};

pub struct Handle {
    pub target : String,
    lib : Option<Library>,
    path : Option<String>,
}

impl Handle {
    pub fn new(target : String, display : &glium::Display) -> Handle {
        let mut h = Handle { target : target, lib : None, path : None };
        h.reload(display);
        h
    }

    fn deinit(&mut self) {
        self.lib.as_ref().map(|lib| {
            let deinit : Symbol<extern "C" fn()> =
                unsafe { lib.get(b"deinit\0").unwrap() };
            deinit();
        });
    }

    pub fn init(&mut self, display : &glium::Display) {
        self.lib.as_ref().map(|lib| {
            let init :  Symbol<extern "C" fn(*const glium::Display)> =
                unsafe { lib.get(b"init\0").unwrap() };
            init(display);
        });
   }

    pub fn draw(&mut self, counter : i32, mut frame : glium::Frame) {
        self.lib.as_ref().map(|lib| {
            let draw :  Symbol<extern "C" fn(i32, *mut glium::Frame)> =
                unsafe { lib.get(b"draw\0").unwrap() };
            draw(counter, &mut frame);
        });
        frame.finish().unwrap();
    }

    pub fn reload(&mut self, display : &glium::Display) {
        self.deinit();
        self.path.as_ref().map(fs::remove_file);

        let new_path = format!("{}.{}", self.target,
                time::SystemTime::now().duration_since(time::UNIX_EPOCH)
                                       .unwrap().as_secs());

        fs::copy(&self.target, &new_path)
            .expect("Could not copy library");
        Command::new("install_name_tool")
            .args(&["-id", &new_path, &new_path])
            .spawn()
            .expect("Failed to call 'install_name_tool'")
            .wait()
            .expect("Failed to rename library with 'install_name_tool'");

        self.lib = Library::new(&new_path).ok();
        self.path = Some(new_path);
        self.init(display);
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        self.path.as_ref().map(fs::remove_file);
    }
}

