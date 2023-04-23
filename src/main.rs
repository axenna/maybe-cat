use maybe_cat::obj::Obj;
use maybe_cat::sim::Sim;
use maybe_cat::vec3::Vec3;
use maybe_cat::force::ForceReg;

fn main() {
    let p = Obj::default_particle();
    let mut mp = Obj::default_mass_particle();
    let mut sim = Sim::default();
    let mut gravity = ForceReg::make_gravity(10.0);


    let pshare = sim.add_obj(p);
    let mpshare = sim.add_obj(mp);

    gravity.add_target(pshare);
    gravity.add_target(mpshare);
    
    let g_share = sim.add_force(gravity);

    sim.run(5);

    

    sim.print_objs();
}
