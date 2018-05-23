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

    // Blend clouds as they go behind the planet, since otherwise
    // they stack up and get too bright.
    float cutoff = 0.4;
    float min_shade = 0.1;
    if (depth > 0) {
        shade *= min_shade;
    } else if (depth > -cutoff) {
        shade *= max(min_shade, pow((-depth) / cutoff, 2.0));
    }

    float n = length((tex_coord - 0.5) * 2.0);
    float circle = 0.1;
    if (n > 1.0) {
        discard;
    } else if (n > circle) {
        shade *= pow((1.0 - n) / (1 - circle), 2.0);
    }

    float r = texture(tex, (tex_coord + vec2(mod(tex_index, 6.0), mod(tex_index, 36.0))) / 6.0).r * shade;
    color_out = vec4(1.0, 1.0, 1.0, r);
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
        for i in 0..100 {
            // Pick a central seed for the cloud on the unit sphere
            let mut v = Vector3::new(1.0, 1.0, 1.0);
            while v.magnitude() > 1.0 {
                v = Vector3::new(jitter(), jitter(), jitter());
            }

            v = v.normalize() * 1.1;
            for j in 0..25 {
                let w = v + Vector3::new(jitter(), jitter(), jitter()) / 10.0;

                // Prevent the clouds from drifting too much on the Z axis
                let m = (w.magnitude() - 1.0) / 10.0  + 1.0;
                let w = array3(w * m / w.magnitude() * v.magnitude());

                verts.push(Vertex {  position: w, offset: [-1f32, -1f32], index: index });
                verts.push(Vertex {  position: w, offset: [ 1f32, -1f32], index: index });
                verts.push(Vertex {  position: w, offset: [ 1f32,  1f32], index: index });

                verts.push(Vertex {  position: w, offset: [-1f32, -1f32], index: index });
                verts.push(Vertex {  position: w, offset: [ 1f32,  1f32], index: index });
                verts.push(Vertex {  position: w, offset: [-1f32,  1f32], index: index });

                index += 1;
            }
        }

        // Build a billowy noise texture; different quads index into
        // different regions on the texture to hide repetition.
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
        let img = RawImage2d::from_raw_rgb(img.into_raw(), image_dimensions);
        let tex = Texture2d::new(facade, img)?;

        let v = VertexBuffer::new(facade, &verts)?;
        let p = Program::from_source(facade, VERTEX_SHADER_SRC,
                                     FRAGMENT_SHADER_SRC, None)?;
        Ok(Clouds{ vbo: v, program: p, tex: tex })
    }

    pub fn draw(&self, mat : Matrix4<f32>, frame : &mut Frame, params : &DrawParameters) {
        let params = DrawParameters {
            depth : Depth {
                test: DepthTest::IfLess,
                write: false,
                .. Default::default()
            },
            blend : draw_parameters::Blend::alpha_blending(),
            .. params.clone()
        };
        let indices = NoIndices(PrimitiveType::TrianglesList);
        let uniforms = uniform! {
            M : array4x4(mat),
            tex : &self.tex,
        };
        let indices = NoIndices(PrimitiveType::TrianglesList);

        frame.draw(&self.vbo, indices, &self.program,
                   &uniforms, &params).unwrap();
    }
}
