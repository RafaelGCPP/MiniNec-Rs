
mod integrate;
mod geometry;   

use geometry::*;

fn main() {
    println!("Hello, world!");

    let antenna = read_antenna_from_file("TestData/antenna.json");
    println!("{:#?}", antenna);
    
}

