use maybe_cat::particle::Particle;
use maybe_cat::sim::Sim;
use maybe_cat::vec3::Vec3;
use maybe_cat::force::ForceReg;

fn main() {
    let p1 = Particle::default();
    let mut sim = Sim::default();
    let gravity = Vec3::new(0.0, -10.0, 0.0);

    let p_share = sim.add_particle(p1);

    let grav_reg = ForceReg::new(gravity, vec![p_share]);
    
    sim.reg_force(grav_reg);
    sim.run(5);
    sim.print_objs();
}
