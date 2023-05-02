use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct CatData {
    pub pos: Vec3,
    pub vel: Vec3,
    pub acc: Vec3,
}

impl CatData {
    pub fn new(pos: Vec3, vel: Vec3, acc: Vec3) -> Self {
        CatData { pos, vel, acc }
    }
    
    pub fn random() -> Self {
        Self::new(Vec3::random(), Vec3::random(), Vec3::random())
    }

    pub fn default() -> Self {
        Self::new(Vec3::zeroes(), Vec3::zeroes(), Vec3::zeroes())
    }

    pub fn integrate(&mut self, t: f32) {
        self.pos.add_scaled(&self.vel, t);
        self.vel.add_scaled(&self.acc, t);
        self.acc = Vec3::zeroes();
    }
}
