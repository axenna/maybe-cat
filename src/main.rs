use maybe_cat::obj::Obj;
use maybe_cat::sim::Sim;
use maybe_cat::vec3::Vec3;
use maybe_cat::force::ForceReg;

fn main() {
    let mut p = Obj::default_particle();
    let mut p2 = Obj::default_particle();

    let five_vec = Vec3::new(5.0, 0.0, 0.0);
    let neg_five_vec = { let mut x = five_vec.clone(); x.a *= -1.0; x};

    let mut p_cd = p.cat_data_mut();
    let mut p2_cd = p2.cat_data_mut();

    p_cd.pos.add(&five_vec);
    p2_cd.pos.add(&neg_five_vec);
    p_cd.vel.add(&neg_five_vec);
    p2_cd.vel.add(&five_vec);


    let mut sim = Sim::default();
    // let mut gravity = ForceReg::make_gravity(10.0);


    let pshare = sim.add_obj(p);
    let p2share = sim.add_obj(p2);

    // gravity.add_target(pshare);
    // gravity.add_target(mpshare);
    
    // let g_share = sim.add_force(gravity);

    sim.run_debug(3);

    sim.print_objs();

}
