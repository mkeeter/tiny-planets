extern crate cgmath;
extern crate glium;

use std::error::Error;

use draw::ocean::Ocean;
use draw::terrain::Terrain;
use draw::atmosphere::Atmosphere;
use draw::clouds::Clouds;

use self::glium::*;
use self::cgmath::{Vector4, Matrix4, Rad, SquareMatrix, Transform, Array};

pub struct Planet
{
    terrain: Option<Terrain>,
    ocean : Option<Ocean>,
    clouds : Option<Clouds>,
    atmosphere : Option<Atmosphere>,
}

impl Planet {
    pub fn new(display : &glium::Display) -> Result<Planet, Box<Error>> {
        let terrain = Terrain::new(display);
        if terrain.is_err() {
            println!("Couldn't construct Terrain: {}", terrain.as_ref().err().unwrap());
        }

        let ocean = Ocean::new(display);
        if ocean.is_err() {
            println!("Couldn't construct Ocean: {}", ocean.as_ref().err().unwrap());
        }

        let atmo = Atmosphere::new(display);
        if atmo.is_err() {
            println!("Couldn't construct Atmosphere: {}", atmo.as_ref().err().unwrap());
        }

        let clouds = Clouds::new(display);
        if clouds.is_err() {
            println!("Couldn't construct Clouds: {}", clouds.as_ref().err().unwrap());
        }

        Ok(Planet { terrain : terrain.ok(),
                    ocean : ocean.ok(),
                    atmosphere : atmo.ok(),
                    clouds: clouds.ok(),
        })
    }

    pub fn draw(&self, counter: i32, frame: &mut Frame, viewport: &Option<Rect>) {

        let mut scale = Vector4::from_value(0.7f32);
        scale.w = 1f32;
        let mut mat = Matrix4::from_diagonal(scale);
        mat.concat_self(&Matrix4::from_angle_y(Rad(counter as f32 / 100f32)));

        let params = glium::DrawParameters {
            viewport: viewport.clone(),
            depth : Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
        self.terrain.as_ref().map(|t| { t.draw(mat, frame, &params); });
        self.ocean.as_ref().map(|o| { o.draw(mat, frame, &params); });

        let params = glium::DrawParameters {
            depth : Depth {
                test: glium::DepthTest::IfLess,
                write: false,
                .. Default::default()
            },
            blend : glium::draw_parameters::Blend::alpha_blending(),
            .. params
        };
        self.clouds.as_ref().map(|c| { c.draw(mat, frame, &params); });

        let params = glium::DrawParameters {
            viewport: viewport.clone(),
            blend : glium::draw_parameters::Blend::alpha_blending(),
            .. Default::default()
        };
        self.atmosphere.as_ref().map(|a| { a.draw(frame, &params) });
    }
}
