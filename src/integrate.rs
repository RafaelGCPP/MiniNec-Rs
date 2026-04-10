pub trait Integrable {
    type Output;
    fn eval(&self, x: f64) -> Self::Output;
}


impl<F, Out> Integrable for F 
where F: Fn(f64) -> Out 
{
    type Output = Out;
    fn eval(&self, x: f64) -> Self::Output {
        self(x)
    }
}


pub fn integrate<T>(f: &T, a: f64, b: f64, steps: usize) -> T::Output
where
    T: Integrable,
    T::Output: std::ops::AddAssign + std::ops::Mul<f64, Output = T::Output> + Default + Copy,
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

fn integrate_step<T>(f: &T, a: f64, b: f64) -> T::Output
where
    T: Integrable,
    T::Output: std::ops::AddAssign + std::ops::Mul<f64, Output = T::Output> + Default + Copy,
{
    let c1 = (b - a) / 2.0;
    let c2 = (b + a) / 2.0;
    quadrature(&|x: f64| f.eval(c1 * x + c2)) * c1
}

// 5 point Gauss-Legendre quadrature in the interval [-1, 1]
fn quadrature<T>(f: &T) -> T::Output
where
    T: Integrable,
    T::Output: std::ops::AddAssign + std::ops::Mul<f64, Output = T::Output> + Default + Copy,
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

    let mut sum = T::Output::default();
    for i in 0..5 {
        sum += f.eval(x[i]) * w[i];
    }
    sum 
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    use num_complex::Complex64;

    #[test]
    fn test_integrate_gauss_quadrature_core() {
        // Teste base: Polinômio de grau 9 deve ser exato (limite da máquina)
        // f(x) = x^9 -> integral de -1 a 1 é 0
        let result = quadrature(&|x: f64| x.powi(9));
        assert!(result.abs() < 1e-15);

        // f(x) = x^8 -> integral de -1 a 1 é 2/9
        let result = quadrature(&|x: f64| x.powi(8));
        assert!((result - 2.0 / 9.0).abs() < 1e-15);
    }

    #[test]
    fn test_integrate_step_real() {
        // f(x) = cos(x) em [-pi/2, pi/2] -> deve ser 2.0
        // Com 1 step (5 pontos), o erro do cosseno fica em torno de 1e-7 a 1e-6
        let result = integrate_step(&|x: f64| x.cos(), -PI / 2.0, PI / 2.0);
        assert!((result - 2.0).abs() < 5e-7);

        // f(x) = x^2 em [0, 3] -> deve ser 9.0
        // Como é polinômio de grau 2, deve ser exato
        let result = integrate_step(&|x: f64| x * x, 0.0, 3.0);
        assert!((result - 9.0).abs() < 1e-14);
    }

    #[test]
    fn test_integrate_multistep_convergence() {
        let a = -PI / 2.0;
        let b = PI / 2.0;
        let target = 2.0;

        // Com 1 step: erro esperado ~1e-7 a 1e-6
        let res1 = integrate(&|x: f64| x.cos(), a, b, 1);
        
        // Com 6-8 steps: deve chegar no limite da precisão f64 (~1e-15)
        let res8 = integrate(&|x: f64| x.cos(), a, b, 8);
        
        assert!((res1 - target).abs() < 5e-7);
        assert!((res8 - target).abs() < 1e-14); // Evitando 1e-16 por segurança contra ruído
    }

    #[test]
    fn test_integrate_complex_integration() {
        // No MiniNEC você terá f(x) = e^(-jkx)
        // Vamos testar integral de e^(i*x) de 0 a PI
        // Integral de e^(ix) = -i * e^(ix) |_0^PI = -i(e^(i*PI) - e^0) = -i(-1 - 1) = 2i
        
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
        // Exemplo de como você usará no MiniNEC
        struct MyKernel { frequency_factor: f64 }
        
        impl Integrable for MyKernel {
            type Output = f64;
            fn eval(&self, x: f64) -> f64 {
                (self.frequency_factor * x).cos()
            }
        }

        let my_k = MyKernel { frequency_factor: 1.0 };
        let result = integrate(&my_k, 0.0, PI, 4);
        
        // integral de cos(x) de 0 a PI é 0
        assert!(result.abs() < 1e-14);
    }
}