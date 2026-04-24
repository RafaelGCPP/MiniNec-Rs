use super::integrate::Integrable;
use nalgebra::{Point3, Vector3};
use num_complex::Complex64;
use std::f64::consts::PI;

/// Structure to integrate the Vector Potential A
/// Represents the interaction of a source pulse arm over an observation point.
pub struct GreenKernel {
    pub observation_point: Point3<f64>,
    pub source_start: Point3<f64>,
    pub source_unit_vector: Vector3<f64>,
    pub radius: f64,
    pub k: f64,
}

impl Integrable for GreenKernel {
    type Output = Complex64;

    fn eval(&self, s: f64) -> Self::Output {
        // r_prime: Current point along the source pulse arm
        // s is the scalar distance from the start of the arm (source_start)
        let r_prime = self.source_start + self.source_unit_vector * s;

        // Euclidean distance between observation and source points
        let dist_sq = (self.observation_point - r_prime).norm_squared();

        // Reduced Kernel Approximation: r = sqrt(dist^2 + a^2)
        // The radius 'a' avoids the singularity when dist -> 0
        let r = (dist_sq + self.radius.powi(2)).sqrt();

        // G(r) = exp(-jkr) / r
        let phase = Complex64::new(0.0, -self.k * r);
        phase.exp() / (4.0 * PI * r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::integrate::integrate;
    use nalgebra::{point, vector};

    #[test]
    fn eval_matches_closed_form_at_arbitrary_point() {
        let kernel = GreenKernel {
            observation_point: point![1.0, -2.0, 0.5],
            source_start: point![-1.0, 0.0, 0.5],
            source_unit_vector: vector![0.0, 2.0, 0.0].normalize(),
            radius: 0.2,
            k: 3.5,
        };

        let s = 0.75;
        let r_prime = kernel.source_start + kernel.source_unit_vector * s;
        let expected_r =
            ((kernel.observation_point - r_prime).norm_squared() + kernel.radius.powi(2)).sqrt();
        let expected =
            Complex64::new(0.0, -kernel.k * expected_r).exp() / (4.0 * PI * expected_r);

        let result = kernel.eval(s);

        assert!((result.re - expected.re).abs() < 1e-14);
        assert!((result.im - expected.im).abs() < 1e-14);
    }

    #[test]
    fn eval_is_regularized_at_self_interaction() {
        let radius = 0.015;
        let kernel = GreenKernel {
            observation_point: point![0.0, 0.0, 0.0],
            source_start: point![0.0, 0.0, 0.0],
            source_unit_vector: vector![1.0, 0.0, 0.0],
            radius,
            k: 0.0,
        };

        let result = kernel.eval(0.0);
        let expected = Complex64::new(1.0 / (4.0 * PI * radius), 0.0);

        assert!((result.re - expected.re).abs() < 1e-14);
        assert!(result.im.abs() < 1e-14);
    }

    #[test]
    fn integral_matches_static_closed_form_for_collinear_geometry() {
        let radius = 0.05;
        let length = 0.8;
        let kernel = GreenKernel {
            observation_point: point![0.0, 0.0, 0.0],
            source_start: point![0.0, 0.0, 0.0],
            source_unit_vector: vector![1.0, 0.0, 0.0],
            radius,
            k: 0.0,
        };

        let result = integrate(&kernel, 0.0, length, 128).unwrap();
        let expected = Complex64::new((length / radius).asinh() / (4.0 * PI), 0.0);

        assert!((result.re - expected.re).abs() < 1e-9);
        assert!(result.im.abs() < 1e-14);
    }
}
