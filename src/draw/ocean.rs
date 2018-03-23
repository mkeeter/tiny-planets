extern crate cgmath;

use draw::icosphere::icosphere;

use std::error::Error;

use glium::*;
use glium::backend::Facade;
use glium::index::{PrimitiveType};

use self::cgmath::conv::*;
use self::cgmath::{Matrix4};

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
struct Vertex {
    position : [f32; 3],
}
implement_vertex!(Vertex, position);

const VERTEX_SHADER_SRC : &'static str = r#"
#version 410

uniform mat4 M;

in vec3 position;

out vec3 frag_normal;

void main() {
    gl_Position = M * vec4(position, 1.0);

    frag_normal = normalize(gl_Position.xyz);
}
"#;

const FRAGMENT_SHADER_SRC : &'static str = r#"
#version 410

in vec3 frag_normal;

out vec4 color_out;

void main()
{
    float shade = dot(frag_normal, normalize(vec3(0.5f, 0.5f, -1.0f)));
    vec3 blue = vec3(0.1, 0.2, 0.5) * shade + vec3(0.01, 0.05, 0.2) * (1 - shade);
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

        let mut buffer : Vec<Vertex> = Vec::new();
        v.iter().for_each(|v| {
            buffer.push(Vertex { position : [v[0], v[1], v[2]] });
        });

        let mut indices : Vec<u32> = Vec::new();
        i.iter().for_each(|i| {
            indices.push(i.x);
            indices.push(i.y);
            indices.push(i.z);
        });

        let v = VertexBuffer::new(facade, &buffer)?;
        let p = Program::from_source(facade, VERTEX_SHADER_SRC,
                                     FRAGMENT_SHADER_SRC, None)?;

        let i = IndexBuffer::new(facade, PrimitiveType::TrianglesList,
                                 &indices)?;
        Ok(Ocean { vbo : v, indices : i, program : p })
    }

    pub fn draw(&self, mat : Matrix4<f32>, frame : &mut Frame, params : &DrawParameters) {
        let uniforms = uniform! {
            M : array4x4(mat),
        };

        frame.draw(&self.vbo, &self.indices, &self.program,
                   &uniforms, params).unwrap();
    }
}

