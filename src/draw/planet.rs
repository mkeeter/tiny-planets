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
        Ok(Planet {
            terrain : Terrain::new(display).map_err(|err| {
                println!("Couldn't construct Terrain: {}", err); }).ok(),
            ocean : Ocean::new(display).map_err(|err| {
                println!("Couldn't construct Ocean: {}", err); }).ok(),
            atmosphere : Atmosphere::new(display).map_err(|err| {
                println!("Couldn't construct Atmosphere: {}", err); }).ok(),
            clouds : Clouds::new(display).map_err(|err| {
                println!("Couldn't construct Clouds: {}", err); }).ok(),
        })
    }

    pub fn draw(&self, counter: i32, frame: &mut Frame, params: &glium::DrawParameters) {

        let mut scale = Vector4::from_value(0.7f32);
        scale.w = 1f32;
        let mut mat = Matrix4::from_diagonal(scale);
        mat.concat_self(&Matrix4::from_angle_y(Rad(counter as f32 / 100f32)));

        self.terrain.as_ref().map(|t| { t.draw(mat, frame, &params); });
        self.ocean.as_ref().map(|o| { o.draw(mat, frame, &params); });
        self.clouds.as_ref().map(|c| { c.draw(mat, frame, &params); });
        self.atmosphere.as_ref().map(|a| { a.draw(frame, &params) });
    }
}
