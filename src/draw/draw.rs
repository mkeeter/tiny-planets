extern crate glium;
extern crate nalgebra;

use draw::icosphere;
use draw::planet::Planet;

use glium::*;
use glium::backend::Facade;
use glium::index::{PrimitiveType, NoIndices};

use self::nalgebra::Vector3;

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
struct Vertex {
    position : [f32; 3],
}
implement_vertex!(Vertex, position);

////////////////////////////////////////////////////////////////////////////////

struct TriangleMesh
{
    verts : Vec<Vector3<f32>>
}

impl TriangleMesh
{
    fn vbo<F>(self, facade : &F, p : PrimitiveType)
        -> (VertexBuffer<Vertex>, NoIndices)
        where F : Facade
    {
        let vs : Vec<Vertex> = self.verts.iter().map(
            |q| { Vertex{position : [q.x, q.y, q.z]} }).collect();

        let v = VertexBuffer::new(facade, &vs).unwrap();
        let i =  glium::index::NoIndices(p);

        (v, i)
    }
}

////////////////////////////////////////////////////////////////////////////////

struct IndexedMesh
{
    verts : Vec<Vector3<f32>>,
    indices : Vec<Vector3<u32>>,
}

impl IndexedMesh {
    fn buffers<F>(self, facade : &F, p : PrimitiveType)
        -> (VertexBuffer<Vertex>, IndexBuffer<u32>)
        where F : Facade
    {
        let vs : Vec<Vertex> = self.verts.iter().map(
            |q| { Vertex{position : [q.x, q.y, q.z]} }).collect();

        let v = VertexBuffer::new(facade, &vs).unwrap();

        let mut is = Vec::new();
        self.indices.iter().for_each(|i| { is.push(i.x); is.push(i.y); is.push(i.z); });
        let i = IndexBuffer::new(facade, p, &is).unwrap();
        (v, i)
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct State
{
    planet : Planet,
    //prog: glium::Program,
    //vbo : glium::VertexBuffer<Vertex>,
    //indices : glium::IndexBuffer<u32>,
}

impl State {
    pub fn new(display : &glium::Display) -> State {
        State { planet : Planet::new(display) }
        /*
        let ico = iicosphere(4);
        let (vbo, indices) = ico.buffers(display, PrimitiveType::TrianglesList);

        println!("Start: {:?}", display.get_framebuffer_dimensions());
        let vertex_shader_src = r#"
            #version 140
            in vec3 position;
            out vec3 frag_pos;
            void main() {
                frag_pos = position;
                gl_Position = vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            in vec3 frag_pos;
            out vec4 color;
            void main() {
                color = vec4(frag_pos.z, frag_pos.z, frag_pos.z, 1.0);
            }
        "#;

        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

        State{vbo : vbo, indices : indices, prog : program}
        */
    }
    pub fn draw(&self, _counter : i32, frame : &mut glium::Frame) {
        let dims = frame.get_dimensions();
        frame.clear_color(0.3, 0.2, 0.4, 1.0);
        frame.clear_depth(-1.0);
        let params = glium::DrawParameters {
            viewport: Some(Rect { left: 0, bottom : 0, width: dims.0*2, height: dims.1*2}),
            depth : Depth {
                test: glium::DepthTest::IfMore,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
        self.planet.draw(frame, &params);
    }
}
