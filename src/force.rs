use crate::vec3::Vec3;
use crate::types::Share;
use crate::obj::Obj;
use std::rc::Rc;


pub struct ForceReg {
    f: Vec3,
    targets: Vec<Share<Obj>>,
}

impl ForceReg {
    pub fn new(f: Vec3, targets: Vec<Share<Obj>>) -> Self {
        ForceReg { f, targets }
    }
    pub fn new_no_targets(f: Vec3) -> Self {
        ForceReg { f, targets: Vec::new() }
    }
    pub fn add_target(&mut self, f: Share<Obj>){
        self.targets.push(f);
    }
    pub fn make_gravity(mag: f32) -> Self{
        Self::new(Vec3::new(0.0, mag * -1.0, 0.0), Vec::new())
    }
    pub fn update(&mut self) {
        self.targets
            .iter_mut()
            .for_each(|x| x.borrow_mut().apply_force(&self.f))
    }
}
