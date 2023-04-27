use crate::force::ForceReg;
use crate::types::Share;
use std::rc::Rc;
use std::cell::RefCell;
use crate::obj::Obj;



pub struct Sim {
    fps: u32,
    size: u32,
    objs: Vec<Share<Obj>>,
    forces: Vec<Share<ForceReg>>,
}

impl Sim {
    pub fn new(
        fps: u32,
        size: u32,
        objs: Vec<Share<Obj>>,
        forces: Vec<Share<ForceReg>>,
    ) -> Self {
        Sim {
            fps,
            size,
            objs,
            forces,
        }
    }

    pub fn empty(fps: u32, size: u32) -> Self {
        Self::new(fps, size, Vec::new(), Vec::new())
    }

    pub fn default() -> Self {
        Self::new(60, 10, Vec::new(), Vec::new())
    }
    
    pub fn add_obj(&mut self, o: Obj) -> Share<Obj>{

        let s = Rc::new(RefCell::new(o));
        
        self.objs.push(s.clone());

        s
    }

    pub fn add_force(&mut self, fr: ForceReg) -> Share<ForceReg>{
        let s = Rc::new(RefCell::new(fr));

        self.forces.push(s.clone());

        s
    }

    pub fn run(&mut self, s: u32){
        for i in 0..(s * self.fps) {
            self.update();
        }
    }

    pub fn run_debug(&mut self, s: u32){
        for i in 0..(s * self.fps) {
            self.print_objs();
            self.update();
        }
    }

    pub fn print_objs(&self) {
        self.objs
            .iter()
            .for_each(|x| println!("{:?}", x))
    }
    
    //brute force collision identification
    pub fn identify_collisions(&self) -> Vec<(Share<Obj>, Share<Obj>)>{
        self.objs
            .iter()
            .enumerate()
            .map(|(i, x)| self.objs
                              .iter()
                              .enumerate()
                              .filter(|(j, y)| *j != i && x.borrow().is_colliding_with(&y.borrow()))
                              .map(|(j, y)| (x.clone(), y.clone()))
                              .collect::<Vec<(Share<Obj>, Share<Obj>)>>())
            .flatten()
            .collect()
    }
    

    pub fn update(&mut self) {
        //apply forces
        self.forces
            .iter_mut()
            .for_each(|x| x.borrow_mut().update());
        
        //integrate objects and make sure they don't go out of bounds
        self.objs
            .iter_mut()
            .for_each(|x| {

                let mut obj = x.borrow_mut();

                obj.integrate(1.0 / self.fps as f32);
                obj.remediate_out_of_bounds(self.size)

            });
        
        //collision remediation
        self.identify_collisions()
            .iter_mut()
            .for_each(|(x, y)| x.borrow_mut().remediate_collision(&y.borrow(), 1.0 / self.fps as f32));
    }
}
