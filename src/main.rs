
mod geometry;
mod physics;

use geometry::*;
use physics::*;

fn main() {
    println!("Hello, world!");

    let problem=load_file("TestData/antenna.json", 100.0).unwrap();

    println!("{:?}",problem.pulses);

    let z= build_z_matrix(&problem);
    let v = build_v_vector (&problem);
    println!("{}", z[(5,5)]);
    println!("{}", problem.wave_number);
    println!("{}", ETA);

    let lu=z.lu();
    let i = lu.solve(&v).unwrap();

    println!("{}", i);

    let i_mag=i.map(|x| x.norm());
    println!("{}", i_mag);
}

