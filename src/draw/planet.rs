extern crate nalgebra as na;

use draw::icosphere::icosphere;

use glium::*;
use glium::backend::Facade;
use glium::index::{PrimitiveType, NoIndices};
use glium::uniforms::EmptyUniforms;

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
struct Vertex {
    position : [f32; 3],
    normal   : [f32; 3],
}
implement_vertex!(Vertex, position, normal);

const VERTEX_SHADER_SRC : &'static str = r#"
#version 410

in vec3 position;
in vec3 normal;

out vec3 frag_position;
out vec3 frag_normal;

void main() {
    frag_position = position;
    frag_normal   = normal;

    gl_Position = vec4(position / 3.0f, 1.0);
}
"#;

const FRAGMENT_SHADER_SRC : &'static str = r#"
#version 410

in vec3 frag_position;
in vec3 frag_normal;

out vec4 color_out;

void main()
{
    color_out = vec4(-frag_normal.z, -frag_normal.z, -frag_normal.z, 1.0f);
}
"#;

pub struct Planet {
    vbo : VertexBuffer<Vertex>,
    program : Program,
}

impl Planet {
    pub fn new<F>(facade : &F) -> Planet
        where F : Facade
    {
        let (v, i) = icosphere(4);

        let mut buffer : Vec<Vertex> = Vec::new();
        i.iter().for_each(|tri| {
            let a = v[tri[0] as usize];
            let b = v[tri[1] as usize];
            let c = v[tri[2] as usize];

            // Convert into vectors to get normal
            let a_ = na::Point3::new(a[0], a[1], a[2]);
            let b_ = na::Point3::new(b[0], b[1], b[2]);
            let c_ = na::Point3::new(c[0], c[1], c[2]);
            let norm_ = (b_ - a_).cross(&(c_ - a_));

            // Then convert back into regular arrays
            let norm = [norm_.x, norm_.y, norm_.z];

            // Store this triangle, with positions and per-vertex normals
            buffer.push(Vertex { position : a, normal : norm });
            buffer.push(Vertex { position : b, normal : norm });
            buffer.push(Vertex { position : c, normal : norm });
        });

        let v = VertexBuffer::new(facade, &buffer).unwrap();
        let p = Program::from_source(facade, VERTEX_SHADER_SRC,
                                     FRAGMENT_SHADER_SRC, None).unwrap();
        Planet { vbo : v, program : p }
    }

    pub fn draw(&self, frame : &mut Frame, params : &DrawParameters) {

        let indices = NoIndices(PrimitiveType::TrianglesList);
        frame.draw(&self.vbo, indices, &self.program,
                   &EmptyUniforms, params).unwrap();
    }
}
