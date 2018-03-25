extern crate cgmath;
extern crate glium;

use draw::planet::Planet;
use draw::ocean::Ocean;
use draw::atmosphere::Atmosphere;
use draw::stars::Stars;
use draw::clouds::Clouds;

use glium::*;
use glium::BlendingFunction::{Min, Max, AlwaysReplace};

use self::cgmath::{Vector4, Matrix4, Rad, SquareMatrix, Transform, Array};

////////////////////////////////////////////////////////////////////////////////

pub struct State
{
    planet : Option<Planet>,
    ocean : Option<Ocean>,
    atmosphere : Option<Atmosphere>,
    stars : Option<Stars>,
    clouds : Option<Clouds>,
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

        let atmo = Atmosphere::new(display);
        if atmo.is_err() {
            println!("Couldn't construct Atmosphere: {}", atmo.as_ref().err().unwrap());
        }

        let stars = Stars::new(display);
        if stars.is_err() {
            println!("Couldn't construct Stars: {}", stars.as_ref().err().unwrap());
        }

        let clouds = Clouds::new(display);
        if clouds.is_err() {
            println!("Couldn't construct Clouds: {}", clouds.as_ref().err().unwrap());
        }

        State { planet : planet.ok(),
                ocean : ocean.ok(),
                atmosphere : atmo.ok(),
                stars : stars.ok(),
                clouds: clouds.ok(),
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

        self.stars.as_ref().map(|a| { a.draw(frame, &params) });

        let mut scale = Vector4::from_value(0.7f32);
        scale.w = 1f32;
        let mut mat = Matrix4::from_diagonal(scale);
        mat.concat_self(&Matrix4::from_angle_y(Rad(counter as f32 / 100f32)));

        self.planet.as_ref().map(|p| { p.draw(mat, frame, &params); });
        self.ocean.as_ref().map(|o| { o.draw(mat, frame, &params); });

        let params = glium::DrawParameters {
            viewport: params.viewport,
            blend : glium::draw_parameters::Blend {
                color : glium::BlendingFunction::Addition {
                    source : glium::LinearBlendingFactor::SourceAlpha,
                    destination : glium::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                alpha : glium::BlendingFunction::Addition {
                    source : glium::LinearBlendingFactor::SourceAlpha,
                    destination : glium::LinearBlendingFactor::DestinationAlpha,
                },
                constant_value: (0.0, 0.0, 0.0, 0.0),
            },

            .. Default::default()
        };
        self.clouds.as_ref().map(|c| { c.draw(mat, frame, &params); });

        let params = glium::DrawParameters {
            viewport: params.viewport,
            blend : glium::draw_parameters::Blend::alpha_blending(),
            .. Default::default()
        };
        self.atmosphere.as_ref().map(|a| { a.draw(frame, &params) });
    }
}
