
mod geometry;
mod physics;

use geometry::*;

fn main() {
    println!("Hello, world!");

    let problem=load_file("TestData/antenna.json", 60.0).unwrap();
    println!("{:#?}", problem);
    
}

