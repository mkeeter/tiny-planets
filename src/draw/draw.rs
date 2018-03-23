extern crate cgmath;
extern crate glium;

use draw::planet::Planet;
use draw::ocean::Ocean;

use glium::*;

use self::cgmath::{Vector4, Matrix4, Rad, SquareMatrix, Transform, Array};

////////////////////////////////////////////////////////////////////////////////

pub struct State
{
    planet : Option<Planet>,
    ocean : Option<Ocean>,
}

impl State {
    pub fn new(display : &glium::Display) -> State {
        let ocean = Ocean::new(display);
        if ocean.is_err() {
            println!("Couldn't construct Ocean: {}", ocean.as_ref().err().unwrap());
        }

        let planet = Planet::new(display);
        if planet.is_err() {
            println!("Couldn't construct Planet: {}", planet.as_ref().err().unwrap());
        }

        State { planet : planet.ok(),
                ocean : ocean.ok(),
        }
    }

    pub fn draw(&self, counter : i32, frame : &mut glium::Frame) {
        let dims = frame.get_dimensions();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame.clear_depth(1.0);

        let params = glium::DrawParameters {
            viewport: Some(Rect { left: 0, bottom : 0, width: dims.0*2, height: dims.1*2}),
            depth : Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        let mut scale = Vector4::from_value(0.5f32);
        scale.w = 1f32;
        let mut mat = Matrix4::from_diagonal(scale);
        mat.concat_self(&Matrix4::from_angle_y(Rad(counter as f32 / 100f32)));

        self.planet.as_ref().map(|p| { p.draw(mat, frame, &params); });
        self.ocean.as_ref().map(|o| { o.draw(mat, frame, &params); });
    }
}
