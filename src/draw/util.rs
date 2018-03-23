pub struct Interpolator {
    pub pts : Vec<[f32;2]>,
}

impl Interpolator {
    pub fn at(&self, input : f32) -> f32 {
        for i in 0..(self.pts.len() - 1) {
            if self.pts[i][0] <= input && input <= self.pts[i + 1][0] {
                let frac = (input - self.pts[i][0]) /
                           (self.pts[i + 1][0] - self.pts[i][0]);
                return frac * (self.pts[i + 1][1] - self.pts[i][1])
                       + self.pts[i][1];
            }
        }
        return 0f32;
    }
}
