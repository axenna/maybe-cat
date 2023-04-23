use crate::cat_data::CatData;
use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub enum Obj {
    Particle(CatData),
    MassParticle(f32, CatData),
}

use Obj::*;

impl Obj {
    pub fn new_particle(cd: CatData) -> Obj {
        Particle(cd)
    }

    pub fn default_particle() -> Obj {
        Self::new_particle(CatData::default())
    }

    pub fn new_mass_particle(inv_m: f32, cd: CatData) -> Obj {
        MassParticle(inv_m, cd)
    }

    pub fn default_mass_particle() -> Obj {
        Self::new_mass_particle(1.0, CatData::default())
    }

    pub fn apply_force(&mut self, f: &Vec3) {
        match self {
            Particle(cd) => cd.acc.add(f),
            MassParticle(invm, cd) => cd.acc.add_scaled(f, *invm),
        }
    }

    pub fn integrate(&mut self, t: f32) {
        match self {
            Particle(cd) => cd.integrate(t),
            MassParticle(_, cd) => cd.integrate(t),
        }
    }

    pub fn cat_data(&self) -> &CatData {
        match self {
            Particle(cd) => cd,
            MassParticle(_, cd) => cd,
        }
    }

    pub fn cat_data_mut(&mut self) -> &mut CatData {
        match self {
            Particle(cd) => cd,
            MassParticle(_, cd) => cd,
        }
    }

    pub fn is_colliding_with(&self, other: &Self) -> bool {
        self.cat_data()
            .pos
            .within_distance(&other.cat_data().pos, 1.0)
    }
}
