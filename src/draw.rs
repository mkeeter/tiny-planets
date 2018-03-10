extern crate glium;
use glium::*;
use glium::backend::Facade;
use glium::index::{PrimitiveType, NoIndices};

extern crate nalgebra;
use self::nalgebra::Vector3;

use std::collections::HashMap;
use std::cmp::{min, max};

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

fn icosphere(level : u8) -> IndexedMesh {
    let p = 1.618033988749894f32;
    let mut vs = vec!(
           Vector3::new(-1f32,  0f32,  p),
           Vector3::new( 1f32, 0f32,  p),
           Vector3::new(-1f32,  0f32, -p),
           Vector3::new( 1f32,  0f32, -p),
           Vector3::new( 0f32,  p,  1f32),
           Vector3::new( 0f32,  p, -1f32),
           Vector3::new( 0f32, -p,  1f32),
           Vector3::new( 0f32, -p, -1f32),
           Vector3::new( p,  1f32,  0f32),
           Vector3::new(-p,  1f32,  0f32),
           Vector3::new( p, -1f32,  0f32),
           Vector3::new(-p, -1f32,  0f32));
    let mut ts = vec!(
           Vector3::new(0,   4,   1),
           Vector3::new(0,   9,   4),
           Vector3::new(9,   5,   4),
           Vector3::new(4,   5,   8),
           Vector3::new(4,   8,   1),
           Vector3::new(8,  10,   1),
           Vector3::new(8,   3,  10),
           Vector3::new(5,   3,   8),
           Vector3::new(5,   2,   3),
           Vector3::new(2,   7,   3),
           Vector3::new(7,  10,   3),
           Vector3::new(7,   6,  10),
           Vector3::new(7,  11,   6),
           Vector3::new(11,  0,   6),
           Vector3::new(0,   1,   6),
           Vector3::new(6,   1,  10),
           Vector3::new(9,   0,  11),
           Vector3::new(9,  11,   2),
           Vector3::new(9,   2,   5),
           Vector3::new(7,   2,  11));

    {
        let mut edge_map = HashMap::new();
        let mut edge = |a, b| {
            let k = (min(a,b), max(a,b));
            if !edge_map.contains_key(&k) {
                edge_map.insert(k, vs.len());
                let mid = (vs[a] + vs[b]) / 2f32;
                vs.push(mid);
            }
            return edge_map.get(&k).unwrap().clone();
        };

        for i in 0..level {
            let mut ts_ = Vec::new();
            ts.iter().for_each(|t| {
                ts_.push(Vector3::new(t[0], edge(t[0], t[1]), edge(t[0], t[2])));
                ts_.push(Vector3::new(t[1], edge(t[1], t[2]), edge(t[1], t[0])));
                ts_.push(Vector3::new(t[2], edge(t[2], t[0]), edge(t[2], t[1])));
                ts_.push(Vector3::new(edge(t[0], t[1]), edge(t[1], t[2]), edge(t[2], t[0])));
            });
            ts = ts_;
        }
    }

    vs = vs.iter().map(|v| { v / (v.norm() * 2f32) }).collect();

    let ts_u : Vec<Vector3<u32>> = ts.iter().map(
               |v| { Vector3::new(v[0] as u32, v[1] as u32, v[2] as u32) })
        .collect();

    IndexedMesh{ verts : vs, indices : ts_u }
}

////////////////////////////////////////////////////////////////////////////////

pub struct State
{
    prog: glium::Program,
    vbo : glium::VertexBuffer<Vertex>,
    indices : glium::IndexBuffer<u32>,
}

impl State {
    pub fn new(display : &glium::Display) -> State {
        let ico = icosphere(2);
        let (vbo, indices) = ico.buffers(display, PrimitiveType::TrianglesList);

        println!("Start: {:?}", display.get_framebuffer_dimensions());
        let vertex_shader_src = r#"
            #version 140
            in vec3 position;
            void main() {
                gl_Position = vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            out vec4 color;
            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

        State{vbo : vbo, indices : indices, prog : program}
    }
    pub fn draw(&self, _counter : i32, frame : &mut glium::Frame) {
        let dims = frame.get_dimensions();
        frame.clear_color(0.3, 0.2, 0.4, 1.0);
        let params = glium::DrawParameters {
            viewport: Some(Rect { left: 0, bottom : 0, width: dims.0*2, height: dims.1*2}),
            .. Default::default()
        };
        frame.draw(&self.vbo, &self.indices, &self.prog, &glium::uniforms::EmptyUniforms, &params).unwrap();
    }
}
