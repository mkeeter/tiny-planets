extern crate cgmath;
extern crate glium;

use draw::planet::Planet;
use draw::stars::Stars;

use glium::*;

use self::cgmath::{Vector4, Matrix4, Rad, SquareMatrix, Transform, Array};

////////////////////////////////////////////////////////////////////////////////

pub struct State
{
    planet : Option<Planet>,
    stars : Option<Stars>,
}

impl State {
    pub fn new(display : &glium::Display) -> State {
        let planet = Planet::new(display);
        if planet.is_err() {
            println!("Couldn't construct Planet: {}", planet.as_ref().err().unwrap());
        }

        let stars = Stars::new(display);
        if stars.is_err() {
            println!("Couldn't construct Stars: {}", stars.as_ref().err().unwrap());
        }

        State { planet : planet.ok(),
                stars : stars.ok(),
        }
    }

    pub fn draw(&self, counter : i32, frame : &mut glium::Frame) {
        let dims = frame.get_dimensions();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame.clear_depth(1.0);

        let viewport = Some(Rect { left: 0, bottom : 0, width: dims.0*2, height: dims.1*2});

        self.stars.as_ref().map(|a| { a.draw(frame, &viewport) });
        self.planet.as_ref().map(|p| { p.draw(counter, frame, &viewport) });
    }
}
