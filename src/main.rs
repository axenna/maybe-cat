use maybe_cat::obj::Obj;
use maybe_cat::sim::Sim;
use maybe_cat::types::Share;
use std::env::args;
use std::process;

fn main() {
    // let mut p = Obj::default_particle();
    // let mut p2 = Obj::default_particle();

    let arguments = args().skip(1).collect::<Vec<String>>();

    if arguments.len() < 3 {
        eprintln!("please enter three arguments for number of particles, simulation size, and seconds to run the simulation");
        process::exit(1);
    }
    
    

    let particles = match &arguments[0].parse::<u32>(){
        Ok(x) => *x,
        _ => {
            eprintln!("please enter a positivie number for the amount of particles in simulation");
            process::exit(1);
        }
    };

    let sim_size = match &arguments[1].parse::<u32>(){
        Ok(x) => *x,
        _ => {
            eprintln!("please enter a positive number for the simulation size");
            process::exit(1);
        }
    };

    let seconds = match &arguments[2].parse::<u32>(){
        Ok(x) => *x,
        _ => {
            eprintln!("please enter a positive number for how long to run the simulation");
            process::exit(1);
        }
    };

    let mut simulation = Sim::empty(60, sim_size);
    
    //generate a list of partilce refs and add them to the simulation
    let _particle_shares = (0..particles).map(|_| simulation.add_obj(Obj::random())).collect::<Vec<Share<Obj>>>();
    
    //produce 60 frames each second for each particle
    simulation.produce_frames(seconds);



    
    // let five_vec = Vec3::new(5.0, 0.0, 0.0);
    // let neg_five_vec = { let mut x = five_vec.clone(); x.a *= -1.0; x};

    // let mut p_cd = p.cat_data_mut();
    // let mut p2_cd = p2.cat_data_mut();

    // p_cd.pos.add(&five_vec);
    // p2_cd.pos.add(&neg_five_vec);
    // p_cd.vel.add(&neg_five_vec);
    // p2_cd.vel.add(&five_vec);


    // let mut sim = Sim::default();
    // let mut gravity = ForceReg::make_gravity(10.0);


    // let pshare = sim.add_obj(p);
    // let p2share = sim.add_obj(p2);

    // gravity.add_target(pshare);
    // gravity.add_target(mpshare);
    
    // let g_share = sim.add_force(gravity);

    // sim.run(3);
    // sim.run_debug(3);

    // sim.print_objs();

}
