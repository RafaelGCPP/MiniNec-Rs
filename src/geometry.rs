#![allow(dead_code)]

use serde::Deserialize;
use std::fs;
// use vector3::Vector3;


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
pub struct Antenna {
    wires: Vec<Wire>,
    ground: String
}

pub fn read_antenna_from_file(filename: &str) -> Antenna {
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    serde_json::from_str(&contents).expect("Should have been able to read Antenna")
}