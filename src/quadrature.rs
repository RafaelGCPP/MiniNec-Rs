
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

pub fn integrate<F>(f: F, a: f64, b: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    let c1 = (b - a) / 2.0;
    let c2 = (b + a) / 2.0;
    c1 * gauss_quadrature(|x| f(c1 * x + c2))
}