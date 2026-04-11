
// Gauss-Legendre quadrature integration module.

/// Nodes (abscissas) and weights of the 5-point Gauss-Legendre quadrature in the interval [-1, 1].
/// Each tuple contains (node, weight) for efficient paired access during integration.
const GAUSS_LEGENDRE_POINTS: [(f64, f64); 5] = [
    (-0.906179845938664, 0.236926885056189), // -1/3 * sqrt(5 + 2 * sqrt(10 / 7)), (322 - 13 * sqrt(70)) / 900
    (-0.538469310105683, 0.478628670499366), // -1/3 * sqrt(5 - 2 * sqrt(10 / 7)), (322 + 13 * sqrt(70)) / 900
    (0.0, 0.568888888888889),                 // 0, 128/225
    (0.538469310105683, 0.478628670499366),  // 1/3 * sqrt(5 - 2 * sqrt(10 / 7)), (322 + 13 * sqrt(70)) / 900
    (0.906179845938664, 0.236926885056189),  // 1/3 * sqrt(5 + 2 * sqrt(10 / 7)), (322 - 13 * sqrt(70)) / 900
];

// Integrable trait is a general interface for any function or struct that can be evaluated at a point.
pub trait Integrable {
    type Output;
    fn eval(&self, x: f64) -> Self::Output;
}

// Implement Integrable for any function that matches the signature Fn(f64) -> Out
impl<F, Out> Integrable for F 
where F: Fn(f64) -> Out 
{
    type Output = Out;
    fn eval(&self, x: f64) -> Self::Output {
        self(x)
    }
}


// Integrate a function `f` over the interval [a, b] using `steps` subdivisions.
pub fn integrate<T>(f: &T, a: f64, b: f64, steps: usize) -> T::Output
where
    T: Integrable,
    T::Output: std::ops::AddAssign + std::ops::Mul<f64, Output = T::Output> + std::ops::Add<Output = T::Output> + Default + Copy,
{
    let mut total = T::Output::default();
    let step_size = (b - a) / steps as f64;
    for i in 0..steps {
        let start = a + i as f64 * step_size;
        let end = start + step_size;
        total += integrate_step(f, start, end);
    }
    total    
}

// Integrate a function `f` over the interval [a, b] using 5-point Gauss-Legendre quadrature.
#[inline]
fn integrate_step<T>(f: &T, a: f64, b: f64) -> T::Output
where
    T: Integrable,
    T::Output: std::ops::AddAssign + std::ops::Mul<f64, Output = T::Output> + std::ops::Add<Output = T::Output> + Default + Copy,
{
    let c1 = (b - a) / 2.0;
    let c2 = (b + a) / 2.0;
    quadrature(&|x: f64| f.eval(c1 * x + c2)) * c1
}

// 5 point Gauss-Legendre quadrature in the interval [-1, 1]
#[inline]
fn quadrature<T>(f: &T) -> T::Output
where
    T: Integrable,
    T::Output: std::ops::AddAssign + std::ops::Mul<f64, Output = T::Output> + std::ops::Add<Output = T::Output> + Default + Copy,
{
    GAUSS_LEGENDRE_POINTS.iter()
        .fold(T::Output::default(), |sum, (x, w)| {
            sum + f.eval(*x) * *w
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    use num_complex::Complex64;

    #[test]
    fn test_integrate_gauss_quadrature_core() {
        // Baseline check: a degree-9 polynomial should be integrated exactly
        // by 5-point Gauss-Legendre on [-1, 1] (up to floating-point roundoff).
        // f(x) = x^9 -> integral from -1 to 1 is 0.
        let result = quadrature(&|x: f64| x.powi(9));
        assert!(result.abs() < 1e-15);

        // f(x) = x^8 -> integral from -1 to 1 is 2/9.
        let result = quadrature(&|x: f64| x.powi(8));
        assert!((result - 2.0 / 9.0).abs() < 1e-15);
    }

    #[test]
    fn test_integrate_step_real() {
        // f(x) = cos(x) on [-pi/2, pi/2] -> expected integral is 2.0.
        // With one 5-point panel, the cosine error is typically around 1e-7 to 1e-6.
        let result = integrate_step(&|x: f64| x.cos(), -PI / 2.0, PI / 2.0);
        assert!((result - 2.0).abs() < 5e-7);

        // f(x) = x^2 on [0, 3] -> expected integral is 9.0.
        // Degree-2 polynomial, so this rule should be exact here.
        let result = integrate_step(&|x: f64| x * x, 0.0, 3.0);
        assert!((result - 9.0).abs() < 1e-14);
    }

    #[test]
    fn test_integrate_multistep_convergence() {
        let a = -PI / 2.0;
        let b = PI / 2.0;
        let target = 2.0;

        // With one panel, expected error is about 1e-7 to 1e-6.
        let res1 = integrate(&|x: f64| x.cos(), a, b, 1);
        
        // With 6-8 panels, this should approach f64 precision limits.
        let res8 = integrate(&|x: f64| x.cos(), a, b, 8);
        
        assert!((res1 - target).abs() < 5e-7);
        // Keep tolerance slightly above machine epsilon to avoid flaky failures.
        assert!((res8 - target).abs() < 1e-14);
    }

    #[test]
    fn test_integrate_complex_integration() {
        // In MiniNEC, kernels often look like f(x) = e^(-j k x).
        // Here we test integral of e^(i x) from 0 to PI.
        // Integral of e^(i x) = -i * e^(i x) |0^PI = -i(e^(i PI) - e^0) = 2i.
        
        let k = 1.0;
        let kernel = |x: f64| {
            Complex64::new(0.0, k * x).exp()
        };

        let result: Complex64 = integrate(&kernel, 0.0, PI, 10);
        let expected = Complex64::new(0.0, 2.0);

        assert!((result.re - expected.re).abs() < 1e-14);
        assert!((result.im - expected.im).abs() < 1e-14);
    }

    #[test]
    fn test_integrate_struct_as_integrable() {
        // Example of using a custom struct as an Integrable kernel.
        struct MyKernel { frequency_factor: f64 }
        
        impl Integrable for MyKernel {
            type Output = f64;
            fn eval(&self, x: f64) -> f64 {
                (self.frequency_factor * x).cos()
            }
        }

        let my_k = MyKernel { frequency_factor: 1.0 };
        let result = integrate(&my_k, 0.0, PI, 4);
        
        // Integral of cos(x) from 0 to PI is 0.
        assert!(result.abs() < 1e-14);
    }
}