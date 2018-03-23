extern crate num_traits;
extern crate cgmath;

use std::collections::HashMap;
use std::cmp::{min, max};

use self::cgmath::Vector3;
use self::cgmath::InnerSpace;

pub fn icosphere(level : u8) -> (Vec<Vector3<f32>>, Vec<Vector3<u32>>) {
    let p = 1.618033988749894f32;
    let vs = vec!(
        [-1f32,  0f32,  p],
        [ 1f32, 0f32,   p],
        [-1f32,  0f32, -p],
        [ 1f32,  0f32, -p],
        [ 0f32,  p,  1f32],
        [ 0f32,  p, -1f32],
        [ 0f32, -p,  1f32],
        [ 0f32, -p, -1f32],
        [ p,  1f32,  0f32],
        [-p,  1f32,  0f32],
        [ p, -1f32,  0f32],
        [-p, -1f32,  0f32]);
    let ts = vec!(
        [0,   4,   1],
        [0,   9,   4],
        [9,   5,   4],
        [4,   5,   8],
        [4,   8,   1],
        [8,  10,   1],
        [8,   3,  10],
        [5,   3,   8],
        [5,   2,   3],
        [2,   7,   3],
        [7,  10,   3],
        [7,   6,  10],
        [7,  11,   6],
        [11,  0,   6],
        [0,   1,   6],
        [6,   1,  10],
        [9,   0,  11],
        [9,  11,   2],
        [9,   2,   5],
        [7,   2,  11]);

    let mut ts : Vec<Vector3<usize>> = ts.iter().map(|t| { Vector3::new(t[0], t[1], t[2]) }).collect();
    let mut vs : Vec<Vector3<f32>> = vs.iter().map(|v| { Vector3::new(v[0], v[1], v[2]) }).collect();

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

        for _i in 0..level {
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

    for v in &mut vs {
        *v = v.normalize();
    }

    // Convert from usize to u32
    let ts : Vec<Vector3<u32>> = ts.iter().map(
               |t| { Vector3::new(t[0] as u32, t[1] as u32, t[2] as u32) })
        .collect();

    (vs, ts)
}

