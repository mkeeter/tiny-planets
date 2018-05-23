use std::f32::consts::PI;
use std::error::Error;

use glium::*;
use glium::backend::Facade;
use glium::uniforms::EmptyUniforms;
use glium::index::{PrimitiveType, NoIndices};

#[derive(Copy, Clone)]
struct Vertex {
    position : [f32; 2],
}
implement_vertex!(Vertex, position);

const VERTEX_SHADER_SRC : &'static str = r#"
#version 410

in vec2 position;
out vec2 frag_pos;

void main()
{
    frag_pos = position;
    gl_Position = vec4(position * 0.75, 0.0, 1.0);
}
"#;

const FRAGMENT_SHADER_SRC : &'static str = r#"
#version 410

in vec2 frag_pos;
out vec4 out_color;

void main()
{
    float r = length(frag_pos);

    float a;
    float cut = 0.95;
    if (r > cut)
        a = exp(-pow(r - cut, 2) / 0.001);
    else
        a = exp(-pow(r - cut, 2) / 0.01);
    out_color = vec4(0.5f, 0.8f, 1.0f, a/5);
}
"#;

pub struct Atmosphere {
    vbo : VertexBuffer<Vertex>,
    program : Program,
}

impl Atmosphere {
    pub fn new<F>(facade : &F) -> Result<Atmosphere, Box<Error>>
        where F : Facade
    {
        let mut buffer : Vec<Vertex> = Vec::new();
        let n = 64;

        buffer.push( Vertex { position : [0f32, 0f32] });
        for i in 0..n {
            let angle = (i as f32) / (n as f32) * 2f32 * PI;
            buffer.push(Vertex { position: [angle.cos(), angle.sin()] });
        }
        let b = buffer[1];
        buffer.push(b);

        let v = VertexBuffer::new(facade, &buffer)?;
        let p = Program::from_source(facade, VERTEX_SHADER_SRC,
                                     FRAGMENT_SHADER_SRC, None)?;
        Ok(Atmosphere { vbo : v, program : p })
    }

    pub fn draw(&self, frame : &mut Frame, params : &DrawParameters) {
        let params = DrawParameters {
            blend : draw_parameters::Blend::alpha_blending(),
            .. params.clone()
        };
        let indices = NoIndices(PrimitiveType::TriangleFan);
        frame.draw(&self.vbo, indices, &self.program, &EmptyUniforms, &params).unwrap();
    }
}
