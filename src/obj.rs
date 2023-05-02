use crate::cat_data::CatData;
use crate::vec3::Vec3;
use rand::prelude::random;

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
    pub fn random() -> Self{
        if random() {
            Particle(CatData::random())
        }
        else {
            Self::new_mass_particle(random::<f32>(), CatData::random())
        }
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

    pub fn inv_mass(&self) -> f32 {
        match self {
            Particle(_) => 1.0,
            MassParticle(inv_m, _) => *inv_m,
        }
    }

    pub fn cat_data_mut(&mut self) -> &mut CatData {
        match self {
            Particle(cd) => cd,
            MassParticle(_, cd) => cd,
        }
    }
    
    //will need to be changed later
    //if any object is within one unit of another it 
    //returns true
    pub fn is_colliding_with(&self, other: &Self) -> bool {
        self.cat_data()
            .pos
            .within_distance(&other.cat_data().pos, 1.0)
    }
    

    //uses impulse momentum, F = (m * v) / t. Implementation not yet working for mass particles
    //ToDo: make sure this works
    pub fn remediate_collision(&mut self, other: &mut Self, t: f32){    
        if !self.is_colliding_with(&other){
            return;
        }

        let ocd = &other.cat_data();
        let scd = &self.cat_data();

        let mut oimpulse = ocd.pos.clone();
        let mut simpulse = scd.pos.clone();
        
        //figure out vector between self and others position
        oimpulse.sub(&scd.pos);
        simpulse.sub(&ocd.pos);
        
        //normalize 
        oimpulse.normalize();
        simpulse.normalize();

        //get the component of others velocity pointing at self
        oimpulse.get_projection(&ocd.vel);

        //don't know if I have this scaling step right yet, but I'm rushing
        oimpulse.scale(self.inv_mass());

        simpulse.get_projection(&scd.vel);

        //don't know if I have this scaling step right yet, but I'm rushing
        simpulse.scale(other.inv_mass());


        self.cat_data_mut().vel.add(&oimpulse);
        self.cat_data_mut().vel.sub(&simpulse);

        other.cat_data_mut().vel.add(&simpulse);
        other.cat_data_mut().vel.sub(&oimpulse);
        
        //make them not collide anymore
        while self.is_colliding_with(&other){
            self.integrate(t);
            other.integrate(t);
        }

    }
    
    //flips any component of velocity that is out of bounds
    pub fn remediate_out_of_bounds(&mut self, size: u32) {

        let cd = self.cat_data_mut();
        let pos = &cd.pos;
        let mut vel = &mut cd.vel;

        let s = size as f32;
        

        if pos.a.abs() >= s{
            vel.a = -vel.a;
        }
        if pos.b.abs() >= s{
            vel.b = -vel.b
        }
        if pos.c.abs() >= s{
            vel.c = -vel.c;
        }
    }
}
