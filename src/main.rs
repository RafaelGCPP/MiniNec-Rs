
mod geometry;
mod physics;

use geometry::*;

fn main() {
    println!("Hello, world!");

    let problem=load_file("TestData/antenna.json").unwrap();
    println!("{:#?}", problem);
    
}

