use nalgebra::{Point3, Vector3};
use num_complex::Complex64;
use super::integrate::Integrable;

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
        phase.exp() / r
    }
}