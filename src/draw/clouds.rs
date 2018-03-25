extern crate image;
extern crate noise;
extern crate rand;
extern crate cgmath;

use std::error::Error;

use glium::*;
use glium::backend::Facade;
use glium::uniforms::EmptyUniforms;
use glium::index::{PrimitiveType, NoIndices};
use glium::texture::{RawImage2d, Texture2d};

use self::image::{GenericImage, ImageBuffer, ConvertBuffer, RgbImage};
use self::noise::NoiseFn;

use self::cgmath::{Matrix4, Vector3, InnerSpace};
use self::cgmath::conv::{array3, array4x4};

use self::rand::distributions::{Range, Sample};
use self::rand::{SeedableRng, ChaChaRng};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    offset: [f32; 2],
    index: i32,
}
implement_vertex!(Vertex, position, offset, index);

const VERTEX_SHADER_SRC : &'static str = r#"
#version 410

uniform mat4 M;

in vec3 position;
in vec2 offset;
in int index;

out vec2 tex_coord;
out float tex_index;
out float depth;

void main() {
    vec4 pos = M * vec4(position, 1.0);
    gl_Position = pos + vec4(offset / 20.0, 0.0, 0.0);
    tex_coord = offset / 2.0 + 0.5;
    depth = pos.z;
    tex_index = index;
}
"#;

const FRAGMENT_SHADER_SRC : &'static str = r#"
#version 410

in vec2 tex_coord;
in float depth;
in float tex_index;

uniform sampler2D tex;

out vec4 color_out;

void main()
{
    float shade = 1.0;

    float cutoff = 0.4;
    if (depth > 0.0) {
        discard;
    }
    else if (depth > -cutoff) {
        shade *= pow((-depth) / cutoff, 2.0);
    }

    vec2 offset = (tex_coord - 0.5) * 2.0;
    float n = length(offset);
    if (n > 1.0) {
        discard;
    } else if (n > 0.5) {
        shade *= pow((1.0 - n) * 2.0, 2.0);
    }

    float r = texture(tex, (tex_coord + vec2(mod(tex_index, 6.0), mod(tex_index, 36.0))) / 6.0).r * shade;
    color_out = vec4(1.0, 1.0, 1.0, r/3.0);
    gl_FragDepth = depth;
}
"#;

pub struct Clouds {
    vbo : VertexBuffer<Vertex>,
    program : Program,
    tex: Texture2d,
}

impl Clouds {
    pub fn new<F>(facade : &F) -> Result<Clouds, Box<Error>>
        where F : Facade
    {
        let mut jitter = {
            let seed: &[_] = &[0];
            let mut rng : ChaChaRng = SeedableRng::from_seed(seed);
            let mut between = Range::new(-1.0, 1.0);
            move || { between.sample(&mut rng) }
        };
        let mut verts : Vec<Vertex> = Vec::new();
        let mut index = 0;
        for i in 0..128 {
            let mut v = Vector3::new(1.0, 1.0, 1.0);
            while v.magnitude() > 1.0 {
                v = Vector3::new(jitter(), jitter(), jitter());
            }
            v = v.normalize() * 1.1;
            for j in 0..16 {
                let w = v + Vector3::new(jitter(), jitter(), jitter()) / 10.0;
                let m = (w.magnitude() - 1.0) / 10.0  + 1.0;
                let w = w * m / w.magnitude() * v.magnitude();
                verts.push(Vertex {  position: array3(w), offset: [-1f32, -1f32], index: index });
                verts.push(Vertex {  position: array3(w), offset: [ 1f32, -1f32], index: index });
                verts.push(Vertex {  position: array3(w), offset: [ 1f32,  1f32], index: index });

                verts.push(Vertex {  position: array3(w), offset: [-1f32, -1f32], index: index });
                verts.push(Vertex {  position: array3(w), offset: [ 1f32,  1f32], index: index });
                verts.push(Vertex {  position: array3(w), offset: [-1f32,  1f32], index: index });

                index += 1;
            }
        }


        let img = {
            let per = noise::ScalePoint::new(noise::Billow::new())
                .set_all_scales(0.1, 0.1, 0.1, 1.0);
            ImageBuffer::from_fn(64, 64, |x, y| {
                let a = per.get([x as f64, y as f64, 0.0]);
                let a = ((a/2.0 + 0.5) * 255.0) as u8;
                image::Rgb([a, a, a])
            })
        };
        let image_dimensions = img.dimensions();
        let raw = img.into_raw();
        let img = RawImage2d::from_raw_rgb(raw, image_dimensions);
        let tex = Texture2d::new(facade, img)?;

        let v = VertexBuffer::new(facade, &verts)?;
        let p = Program::from_source(facade, VERTEX_SHADER_SRC,
                                     FRAGMENT_SHADER_SRC, None)?;
        Ok(Clouds{ vbo: v, program: p, tex: tex })
    }

    pub fn draw(&self, mat : Matrix4<f32>, frame : &mut Frame, params : &DrawParameters) {
        let indices = NoIndices(PrimitiveType::TrianglesList);
        let uniforms = uniform! {
            M : array4x4(mat),
            tex : &self.tex,
        };
        let indices = NoIndices(PrimitiveType::TrianglesList);

        frame.draw(&self.vbo, indices, &self.program,
                   &uniforms, params).unwrap();
    }
}
