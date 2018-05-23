extern crate cgmath;
extern crate noise;

use draw::icosphere::icosphere;

use std::error::Error;

use glium::*;
use glium::backend::Facade;
use glium::index::{PrimitiveType};

use self::cgmath::conv::*;
use self::cgmath::{Matrix4};

use self::noise::NoiseFn;

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
struct Vertex {
    position : [f32; 3],
    shade : f32,
}
implement_vertex!(Vertex, position, shade);

const VERTEX_SHADER_SRC : &'static str = r#"
#version 410

uniform mat4 M;

in vec3 position;
in float shade;

out vec3 frag_normal;
out float frag_shade;

void main() {
    gl_Position = M * vec4(position, 1.0);

    frag_normal = normalize(gl_Position.xyz);
    frag_shade = shade;
}
"#;

const FRAGMENT_SHADER_SRC : &'static str = r#"
#version 410

in vec3 frag_normal;
in float frag_shade;

out vec4 color_out;

void main()
{
    float shade = dot(frag_normal, normalize(vec3(0.5f, 0.5f, -1.0f))) * 0.8;
    vec3 blue = vec3(0.1, 0.2, 0.5) * shade + vec3(0.01, 0.05, 0.2) * (1 - shade);

    if (shade + frag_shade * 0.03 > 0.79) {
        blue += vec3(0.4, 0.4, 0.2);
    }
    else if (shade + frag_shade * 0.04 > 0.75) {
        blue += vec3(0.08, 0.1, 0.03);
    }
    else
    {
        blue += frag_shade * 0.02;
    }

    color_out = vec4(blue, 1.0f);
}
"#;

pub struct Ocean {
    vbo : VertexBuffer<Vertex>,
    indices : IndexBuffer<u32>,
    program : Program,
}

impl Ocean {
    pub fn new<F>(facade : &F) -> Result<Ocean, Box<Error>>
        where F : Facade
    {
        let (v, i) = icosphere(5);


        let per = noise::ScalePoint::new(noise::Perlin::new())
            .set_all_scales(20.0, 20.0, 20.0, 1.0);

        let mut buffer : Vec<Vertex> = Vec::new();
        v.iter().for_each(|v| {
            buffer.push(Vertex { position : [v[0] as f32, v[1] as f32, v[2] as f32], shade : per.get([v[0], v[1], v[2]]) as f32 });
        });

        let mut indices : Vec<u32> = Vec::new();
        i.iter().for_each(|i| {
            indices.push(i.x as u32);
            indices.push(i.y as u32);
            indices.push(i.z as u32);
        });

        let v = VertexBuffer::new(facade, &buffer)?;
        let p = Program::from_source(facade, VERTEX_SHADER_SRC,
                                     FRAGMENT_SHADER_SRC, None)?;

        let i = IndexBuffer::new(facade, PrimitiveType::TrianglesList,
                                 &indices)?;
        Ok(Ocean { vbo : v, indices : i, program : p })
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

        let uniforms = uniform! {
            M : array4x4(mat),
        };

        frame.draw(&self.vbo, &self.indices, &self.program,
                   &uniforms, &params).unwrap();
    }
}

