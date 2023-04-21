use crate::cat_data::CatData;
use crate::force::Forceable;
use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Particle(CatData);

impl Forceable for Particle {
    fn apply_force(&mut self, f: &Vec3) {
        self.0.apply_force(f);
    }
}

impl Particle {
    pub fn new(cd: CatData) -> Self {
        Particle(cd)
    }

    pub fn default() -> Self {
        Particle(CatData::default())
    }

    pub fn integrate(&mut self, t: f32){
        self.0.integrate(t);
    }
}

#[derive(Debug, Clone)]
pub struct MassParticle {
    inv_m: f32,
    cd: CatData,
}

impl Forceable for MassParticle {
    fn apply_force(&mut self, f: &Vec3) {
        self.cd.add_acc_scaled(f, self.inv_m);
    }
}

impl MassParticle {
    pub fn new(inv_m: f32, cd: CatData) -> Self {
        MassParticle { inv_m, cd }
    }
}
