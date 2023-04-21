use crate::types::ShareVec;
use crate::vec3::Vec3;

pub trait Forceable {
    fn apply_force(&mut self, f: &Vec3);
}

pub struct ForceReg {
    f: Vec3,
    targets: ShareVec<dyn Forceable>,
}

impl ForceReg {
    pub fn new(f: Vec3, targets: ShareVec<dyn Forceable>) -> Self {
        ForceReg { f, targets }
    }
    pub fn update(&mut self) {
        self.targets
            .iter_mut()
            .for_each(|x| x.borrow_mut().apply_force(&self.f));
    }
}
