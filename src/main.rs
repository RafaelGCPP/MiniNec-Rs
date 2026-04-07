use std::fs;
use vector3::Vector3;
use serde::Deserialize;

#[derive(Deserialize,Debug)]
struct Point {
    x:f64,
    y:f64,
    z:f64
}

#[derive(Deserialize,Debug)]
struct Wire {
    start: Point,
    end: Point,
    diameter: f64
}

#[derive(Deserialize, Debug)]
struct Antenna {
    wires: Vec<Wire>,
    ground: String
}

fn main() {
    println!("Hello, world!");


    let contents = fs::read_to_string("TestData/antenna.json")
        .expect("Should have been able to read the file");

    let antenna: Antenna=serde_json::from_str(&contents).expect("Should have been able to read Antenna");
    println!("{:?}",antenna);
}
