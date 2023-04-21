use crate::force::ForceReg;
use crate::particle::{MassParticle, Particle};
use crate::types::{ShareVec, Share};
use std::rc::Rc;
use std::cell::RefCell;


pub struct Sim {
    fps: u32,
    particles: ShareVec<Particle>,
    regs: Vec<ForceReg>,
}

impl Sim {
    pub fn new(
        fps: u32,
        particles: ShareVec<Particle>,
        regs: Vec<ForceReg>,
    ) -> Self {
        Sim {
            fps,
            particles,
            regs,
        }
    }

    pub fn default() -> Self {
        Sim {
            fps: 60,
            particles: Vec::new(),
            regs: Vec::new(),
        }
    }

    pub fn add_particle(&mut self, p: Particle) -> Share<Particle>{
        let r = Rc::new(RefCell::new(p));

        self.particles.push(Rc::clone(&r));

        r
    }

    pub fn reg_force(&mut self, fr: ForceReg) {
        self.regs.push(fr);
    }

    pub fn run(&mut self, s: u32){
        for i in 0..(s * self.fps) {
            println!("{:?}", i);
            self.step();
        }
    }

    pub fn print_objs(&self) {
        self.particles
            .iter()
            .for_each(|x| println!("{:?}", x.borrow()));
    }

    pub fn step(&mut self) {
        self.print_objs();
        self.regs.iter_mut().for_each(|x| x.update());
        self.print_objs();
        self.particles.iter_mut().for_each(|x| x.borrow_mut().integrate(1.0 / self.fps as f32));
        self.print_objs();
    }
}
