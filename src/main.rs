mod geometry;
mod quadrature;


fn main() {
    println!("Hello, world!");

    let antenna = geometry::read_antenna_from_file("TestData/antenna.json");
    println!("{:#?}", antenna);
    
}