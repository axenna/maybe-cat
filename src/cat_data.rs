use crate::force::Forceable;
use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct CatData {
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
}

impl Forceable for CatData {
    fn apply_force(&mut self, f: &Vec3,) {
        self.acc.add(f);
    }
}

impl CatData {
    pub fn new(pos: Vec3, vel: Vec3, acc: Vec3) -> Self {
        CatData { pos, vel, acc }
    }

    pub fn default() -> Self {
        Self::new(Vec3::zeroes(), Vec3::zeroes(), Vec3::zeroes())
    }

    pub fn integrate(&mut self, t: f32) {
        self.pos.add_scaled(&self.vel, t);
        self.vel.add_scaled(&self.acc, t);
        self.acc = Vec3::zeroes();
    }

    //expose acceleration to modify catdata
    pub fn add_acc_scaled(&mut self, other: &Vec3, s: f32) {
        self.acc.add_scaled(other, s);
    }
}
