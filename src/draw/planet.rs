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
use self::cgmath::{Matrix4, InnerSpace};
use self::rand::distributions::{Range, Sample};
use self::rand::{SeedableRng, ChaChaRng};

use self::noise::NoiseFn;

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
struct Vertex {
    position : [f32; 3],
    normal   : [f32; 3],
    z_offset : f32,
}
implement_vertex!(Vertex, position, normal, z_offset);

const VERTEX_SHADER_SRC : &'static str = r#"
#version 410

uniform mat4 M;

in vec3 position;
in vec3 normal;
in float z_offset;

out vec3 frag_position;
out vec3 frag_normal;
out float frag_z;
out float frag_z_offset;

void main() {
    gl_Position = M * vec4(position, 1.0);

    frag_position = position;
    frag_normal   = (M * vec4(normal, 0.0)).xyz;
    frag_z = length(position);
    frag_z_offset = z_offset / 10.0f;
}
"#;

const FRAGMENT_SHADER_SRC : &'static str = r#"
#version 410

in vec3 frag_position;
in vec3 frag_normal;
in float frag_z;
in float frag_z_offset;

out vec4 color_out;

void main()
{
    vec3 color;

    const vec3 beach = vec3(0.8, 0.7, 0.4);
    const vec3 snow = vec3(0.8, 0.8, 0.8);
    const vec3 rock = vec3(0.5, 0.4, 0.3);
    const vec3 grass = vec3(0.3, 0.7, 0.3);

    // Special-casing for beaches and mountain peaks
    if (frag_z < 1.01)
    {
        color = beach;
    }
    else if (frag_z > 1.15)
    {
        color = snow;
    }
    else
    {
        float fz = frag_z + frag_z_offset;
        if (fz > 1.15)
            color = snow;
        else if (fz > 1.05)
            color = rock;
        else
            color = grass;
    }

    color_out = vec4(frag_normal.z * color, 1.0f);
}
"#;

pub struct Planet {
    vbo : VertexBuffer<Vertex>,
    program : Program,
}

impl Planet {
    pub fn new<F>(facade : &F) -> Result<Planet, Box<Error>>
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

        let interp = Interpolator { pts: vec!(
                [-1f32, -1f32],
                [ 0f32,  0f32],
                [ 0.6f32,  0.2f32],
                [ 1f32,  1f32],
                ) };

        for i in 0..v.len() {
            // Scale based on Perlin noise field
            let offset = curved.get([v[i][0] as f64, v[i][1] as f64, v[i][2] as f64]) as f32;
            v[i] *= offset / 5f32 + 1f32;

            // Add a little random jitter
            v[i].x += jitter();
            v[i].y += jitter();
            v[i].z += jitter();
        }

        // Per-vertex offsets to jitter biomes up and down
        let biome_offsets : Vec<f32> = v.iter().map(|v| {
            per.get([(5f32 * v[0]) as f64, (5f32 * v[1]) as f64, (5f32 * v[2]) as f64]) as f32
        }).collect();

        let mut buffer : Vec<Vertex> = Vec::new();
        i.iter().for_each(|tri| {
            let a = v[tri[0] as usize];
            let b = v[tri[1] as usize];
            let c = v[tri[2] as usize];

            // Find the normal
            let norm = array3((b - a).cross(c - a).normalize());

            // Store this triangle, with positions and per-vertex normals
            buffer.push(Vertex { position : array3(a), normal : norm, z_offset : biome_offsets[tri[0] as usize] });
            buffer.push(Vertex { position : array3(b), normal : norm, z_offset : biome_offsets[tri[1] as usize] });
            buffer.push(Vertex { position : array3(c), normal : norm, z_offset : biome_offsets[tri[2] as usize] });
        });

        let v = VertexBuffer::new(facade, &buffer)?;
        let p = Program::from_source(facade, VERTEX_SHADER_SRC,
                                     FRAGMENT_SHADER_SRC, None)?;
        Ok(Planet { vbo : v, program : p })
    }

    pub fn draw(&self, mat : Matrix4<f32>, frame : &mut Frame, params : &DrawParameters) {

        let indices = NoIndices(PrimitiveType::TrianglesList);
        let uniforms = uniform! {
            M : array4x4(mat),
        };

        frame.draw(&self.vbo, indices, &self.program,
                   &uniforms, params).unwrap();
    }
}
