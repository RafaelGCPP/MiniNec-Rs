mod geometry;
mod quadrature;


fn main() {
    println!("Hello, world!");

    let antenna = geometry::read_antenna_from_file("TestData/antenna.json");
    println!("{:#?}", antenna);
    
    test_quadrature();

}

fn test_quadrature() {
    let pi = std::f64::consts::PI;

    let x: f64 = 3.0; // This is just to show the expected value of the integral
    let result = quadrature::integrate(|x| x * x, 0.0, x);
    println!("Integral of x^2 from 0 to {} is approximately: {}, and it should be close to {}", x, result, x.powi(3) / 3.0);

    let result = quadrature::gauss_quadrature(|x| (x * pi/2.0).cos()*pi/2.0);
    println!("Integral of cos(x * pi/2)*pi/2 from -1 to 1 is approximately: {}, and it should be close to {}", result, 2.0 * (pi/2.0).sin()); 
}