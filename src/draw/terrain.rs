extern crate cgmath;
extern crate noise;
extern crate rand;

use std::error::Error;

use draw::icosphere::icosphere;
use draw::util::Interpolator;

use glium::*;
use glium::backend::Facade;
use glium::index::{PrimitiveType, NoIndices};

use self::cgmath::conv::*;
use self::cgmath::{Matrix4, Vector3, InnerSpace};
use self::rand::distributions::{Range, Sample};
use self::rand::{SeedableRng, ChaChaRng};

use self::noise::NoiseFn;

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
struct Vertex {
    position : [f32; 3],
    normal   : [f32; 3],
    color    : [f32; 3],
}
implement_vertex!(Vertex, position, normal, color);

const VERTEX_SHADER_SRC : &'static str = r#"
#version 410

uniform mat4 M;

in vec3 position;
in vec3 normal;
in vec3 color;

out vec3 frag_position;
out vec3 frag_normal;
out vec3 frag_color;

void main() {
    gl_Position = M * vec4(position, 1.0);

    frag_position = position;
    frag_normal   = (M * vec4(normal, 0.0)).xyz;
    frag_color = color;
}
"#;

const FRAGMENT_SHADER_SRC : &'static str = r#"
#version 410

in vec3 frag_position;
in vec3 frag_normal;
in vec3 frag_color;

out vec4 color_out;

void main()
{
    color_out = vec4(frag_normal.z * frag_color, 1.0f);
}
"#;

pub struct Terrain {
    vbo : VertexBuffer<Vertex>,
    program : Program,
}

impl Terrain {
    pub fn new<F>(facade : &F) -> Result<Terrain, Box<Error>>
        where F : Facade
    {
        let (mut v, i) = icosphere(5);
        let per = noise::ScalePoint::new(noise::Perlin::new())
            .set_all_scales(3.0, 3.0, 3.0, 1.0);
        let curved = noise::Curve::new(&per)
            .add_control_point(-2.0, -2.0)
            .add_control_point(-1.0, -1.0)
            .add_control_point(-0.5, -0.5)
            .add_control_point( 0.0,  0.0)
            .add_control_point( 0.6,  0.2)
            .add_control_point( 1.0,  1.0);

        let seed: &[_] = &[0];
        let mut rng : ChaChaRng = SeedableRng::from_seed(seed);
        let mut between = Range::new(-0.01, 0.01);
        let mut jitter = || { between.sample(&mut rng) };

        for i in 0..v.len() {
            // Scale based on Perlin noise field
            let offset = curved.get([v[i][0], v[i][1], v[i][2]]);
            v[i] *= offset / 8.0 + 1.0;

            // Add a little random jitter
            v[i].x += jitter();
            v[i].y += jitter();
            v[i].z += jitter();
        }

        let mut buffer : Vec<Vertex> = Vec::new();
        i.iter().for_each(|tri| {
            let a = v[tri[0]];
            let b = v[tri[1]];
            let c = v[tri[2]];

            let array3f = |v : Vector3<f64>| { array3([v[0] as f32, v[1] as f32, v[2] as f32]) };

            // Find the normal
            let norm = array3f((b - a).cross(c - a).normalize());

            // Biome colors (RGB)
            let beach = [0.8, 0.7, 0.4];
            let snow = [0.8, 0.8, 0.8];
            let rock = [0.5, 0.4, 0.3];
            let grass = [0.2, 0.6, 0.2];

            let center = ((a + b + c) / 3.0).magnitude();
            let color : Vector3<f64>;
            let color =
                if center < 1.005 {
                    beach
                } else if center < 1.03 {
                    let mut g = grass;
                    g[0] += 10f32 * jitter() as f32;
                    g[1] += 10f32 * jitter() as f32;
                    g[2] += 10f32 * jitter() as f32;
                    g
                } else if center < 1.08 {
                    rock
                } else {
                    snow
                };


            // Store this triangle, with positions and per-vertex normals
            buffer.push(Vertex { position : array3f(a), normal : norm, color : color });
            buffer.push(Vertex { position : array3f(b), normal : norm, color : color });
            buffer.push(Vertex { position : array3f(c), normal : norm, color : color });
        });

        let v = VertexBuffer::new(facade, &buffer)?;
        let p = Program::from_source(facade, VERTEX_SHADER_SRC,
                                     FRAGMENT_SHADER_SRC, None)?;
        Ok(Terrain { vbo : v, program : p })
    }

    pub fn draw(&self, mat : Matrix4<f32>, frame : &mut Frame, params : &DrawParameters) {

        let params = DrawParameters {
            depth : Depth {
                test: DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. params.clone()
        };

        let indices = NoIndices(PrimitiveType::TrianglesList);
        let uniforms = uniform! {
            M : array4x4(mat),
        };

        frame.draw(&self.vbo, indices, &self.program,
                   &uniforms, &params).unwrap();
    }
}
