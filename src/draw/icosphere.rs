extern crate num_traits;

use std::collections::HashMap;
use std::cmp::{min, max};
use self::num_traits::pow;

fn center(a : [f32; 3], b : [f32; 3]) -> [f32; 3] {
    [(a[0] + b[0]) / 2f32,
     (a[1] + b[1]) / 2f32,
     (a[2] + b[2]) / 2f32]
}

pub fn icosphere(level : u8) -> (Vec<[f32;3]>, Vec<[u32;3]>) {
    let p = 1.618033988749894f32;
    let mut vs = vec!(
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
    let mut ts = vec!(
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

    {
        let mut edge_map = HashMap::new();
        let mut edge = |a, b| {
            let k = (min(a,b), max(a,b));
            if !edge_map.contains_key(&k) {
                edge_map.insert(k, vs.len());
                let mid = center(vs[a], vs[b]);
                vs.push(mid);
            }
            return edge_map.get(&k).unwrap().clone();
        };

        for i in 0..level {
            let mut ts_ = Vec::new();
            ts.iter().for_each(|t| {
                ts_.push([t[0], edge(t[0], t[1]), edge(t[0], t[2])]);
                ts_.push([t[1], edge(t[1], t[2]), edge(t[1], t[0])]);
                ts_.push([t[2], edge(t[2], t[0]), edge(t[2], t[1])]);
                ts_.push([edge(t[0], t[1]), edge(t[1], t[2]), edge(t[2], t[0])]);
            });
            ts = ts_;
        }
    }

    for v in &mut vs {
        let length = (pow(v[0], 2) + pow(v[1], 2) + pow(v[2], 2)).sqrt();
        *v = [v[0] / length, v[1] / length, v[2] / length];
    }

    let ts_u : Vec<[u32; 3]> = ts.iter().map(
               |t| { [t[0] as u32, t[1] as u32, t[2] as u32] })
        .collect();

    (vs, ts_u)
}

