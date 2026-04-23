
mod integrate;
mod geometry;

use geometry::*;

fn main() {
    println!("Hello, world!");

    load_file("TestData/antenna.json").unwrap();
    //println!("{:#?}", antenna);
    
}

