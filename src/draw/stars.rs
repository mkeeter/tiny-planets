extern crate glium;
extern crate image;
extern crate noise;

use std::error::Error;

use glium::*;
use glium::backend::Facade;
use glium::uniforms::EmptyUniforms;
use glium::index::{PrimitiveType, NoIndices};
use glium::texture::{RawImage2d, Texture2d};

use self::image::{GenericImage, ImageBuffer, ConvertBuffer, RgbImage};
use self::noise::NoiseFn;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

const VERTEX_SHADER_SRC : &'static str = r#"
#version 410

in vec2 position;
out vec2 frag_position;

void main() {
    gl_Position = vec4(position, 0.99, 1.0);
    frag_position = position / 2.0 + 0.5;
}
"#;

const FRAGMENT_SHADER_SRC : &'static str = r#"
#version 410

in vec2 frag_position;
uniform sampler2D tex;

out vec4 color_out;

void main()
{
    float r = texture(tex, frag_position).r;

    float cutoff = 0.67;
    if (r < cutoff) {
        r = 0;
    } else {
        r = (r - cutoff) / (1 - cutoff);
    }

    color_out = vec4(r, r, r, 1.0f);
}
"#;

pub struct Stars {
    vbo : VertexBuffer<Vertex>,
    program : Program,
    tex: Texture2d,
}

impl Stars {
    pub fn new<F>(facade : &F) -> Result<Stars, Box<Error>>
        where F : Facade
    {
        let per = noise::ScalePoint::new(noise::Perlin::new())
            .set_all_scales(0.2, 0.2, 0.2, 1.0);
        let img = ImageBuffer::from_fn(512, 512, |x, y| {
            let a = per.get([x as f64, y as f64, 0.0]);
            let a = ((a/2.0 + 0.5) * 255.0) as u8;
            image::Luma([a])
        });
        let img = image::imageops::blur(&img, 2f32);
        let img : RgbImage = img.convert();
        let image_dimensions = img.dimensions();
        let raw = img.into_raw();
        let img = RawImage2d::from_raw_rgb(raw, image_dimensions);
        let tex = Texture2d::new(facade, img)?;

        let shape = vec!(
            Vertex { position: [-1.0, -1.0] },
            Vertex { position: [-1.0,  1.0] },
            Vertex { position: [ 1.0,  1.0] },
            Vertex { position: [ 1.0, -1.0] },
        );
        let vbo = VertexBuffer::new(facade, &shape)?;
        let p = Program::from_source(facade, VERTEX_SHADER_SRC,
                                     FRAGMENT_SHADER_SRC, None)?;

        Ok(Stars{ vbo: vbo, program: p, tex: tex })
    }

    pub fn draw(&self, frame : &mut Frame, params: &glium::DrawParameters) {
        let uniforms = uniform! {
            tex: &self.tex,
        };

        let indices = NoIndices(PrimitiveType::TriangleFan);
        frame.draw(&self.vbo, &indices, &self.program, &uniforms, params);
    }
}
