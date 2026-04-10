
// 5 point Gauss-Legendre quadrature in the interval [-1, 1]
pub fn gauss_quadrature<F>(f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    let x = [
        -0.906179845938664, // -1/3 * sqrt(5 + 2 * sqrt(10 / 7))
        -0.538469310105683, // -1/3 * sqrt(5 - 2 * sqrt(10 / 7))
        0.0,
        0.538469310105683, // 1/3 * sqrt(5 - 2 * sqrt(10 / 7))
        0.906179845938664, // 1/3 * sqrt(5 + 2 * sqrt(10 / 7))
    ];
    let w = [
        0.236926885056189, // (322 - 13 * sqrt(70)) / 900
        0.478628670499366, // (322 + 13 * sqrt(70)) / 900
        0.568888888888889, // 128/225
        0.478628670499366, // (322 + 13 * sqrt(70)) / 900
        0.236926885056189, // (322 - 13 * sqrt(70)) / 900
    ];

    let mut sum = 0.0;
    for i in 0..5 {
        sum += w[i] * f(x[i]);
    }
    sum 
}

fn integrate_step<F>(f: F, a: f64, b: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    let c1 = (b - a) / 2.0;
    let c2 = (b + a) / 2.0;
    c1 * gauss_quadrature(|x| f(c1 * x + c2))
}

pub fn integrate<F>(f: F, a: f64, b: f64, steps: usize) -> f64
where F: Fn(f64) -> f64 + Copy {
    let step_size = (b - a) / steps as f64;
    let mut total = 0.0;
    for i in 0..steps {
        let start = a + i as f64 * step_size;
        let end = start + step_size;
        total += integrate_step(f, start, end);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadrature_gauss_quadrature() {
        // integral of f(x) = cos(x* pi/2) * pi/2 in [-1, 1] should be 2.0
        let result = gauss_quadrature(|x| (x * std::f64::consts::PI / 2.0).cos() * std::f64::consts::PI / 2.0);
        assert!((result - 2.0).abs() < 1e-6);

        //integral of f(x) = x^2 in [-1, 1] should be 2/3
        let result = gauss_quadrature(|x| x * x);
        assert!((result - 2.0 / 3.0).abs() < 1e-15);
    }

    #[test]
    fn test_quadrature_integrate_step() {

        // integral of f(x) = cos(x)  in [-pi/2, pi/2] should be 2.0
        let result = integrate_step(|x| x.cos(), -std::f64::consts::PI / 2.0, std::f64::consts::PI / 2.0);
        assert!((result - 2.0).abs() < 1e-6);


        // integral of f(x) = x in [-1, 1] should be 0.0
        let result = integrate_step(|x| x, -1.0, 1.0);
        assert!(result.abs() < 1e-16);

        // integral of f(x) = x^2 in [0, 3] should be 9.0
        let result = integrate_step(|x| x * x, 0.0, 3.0);
        assert!((result - 9.0).abs() < 1e-14);
    }
    #[test]
    fn test_quadrature_integrate() {

        // integral of f(x) = cos(x)  in [-pi/2, pi/2] should be 2.0
        let result = integrate(|x| x.cos(), -std::f64::consts::PI / 2.0, std::f64::consts::PI / 2.0, 8);
        assert!((result - 2.0).abs() < 1e-15);

        // integral of f(x) = x^2 in [0, 3] should be 9.0
        let result = integrate(|x| x * x, 0.0, 3.0, 4);
        println!("{:e}",(result - 9.0).abs() );

        assert!((result - 9.0).abs() < 1e-14);
    }

}

