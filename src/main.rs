mod geometry_file;
mod integrate;

use geometry_file::*;

fn main() {
    println!("Hello, world!");

    let antenna = read_antenna_from_file("TestData/antenna.json");
    println!("{:#?}", antenna);
    
}

