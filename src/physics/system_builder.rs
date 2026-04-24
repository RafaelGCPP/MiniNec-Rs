use super::ETA;
use super::integrate::{Integrable, integrate};
use super::kernel::GreenKernel;
use crate::geometry::model::{Problem, Pulse};
use nalgebra::{DMatrix, Point3, Vector3};
use num_complex::Complex64;

pub fn build_z_matrix(problem: &Problem) -> DMatrix<Complex64> {
    let n = problem.pulses.len();
    let k = problem.wave_number;

    DMatrix::from_fn(n, n, |i, j| {
        let p_i = &problem.pulses[i];
        let p_j = &problem.pulses[j];
        calculate_pulse_interaction(p_i, p_j, k)
    })
}

pub fn build_v_vector(problem: &Problem) -> DMatrix<Complex64> {
    let n = problem.pulses.len();

    DMatrix::from_fn(n, 1, |i, j| problem.pulses[i].voltage_source)
}

fn calculate_pulse_interaction(p_i: &Pulse, p_j: &Pulse, k: f64) -> Complex64 {
    // 1. Configuração dos Kernels (Fonte j)
    let kernel_in = GreenKernel {
        observation_point: p_i.center_node,
        source_start: p_j.seg_in_midpoint,
        source_unit_vector: p_j.seg_in_unit,
        radius: p_j.seg_in_radius,
        k,
    };
    let kernel_out = GreenKernel {
        observation_point: p_i.center_node,
        source_start: p_j.center_node,
        source_unit_vector: p_j.seg_out_unit,
        radius: p_j.seg_out_radius,
        k,
    };

    // Integrais do potencial vetorial (A)
    let a_in = integrate(&kernel_in, 0.0, p_j.seg_in_half_length, 8).unwrap();
    let a_out = integrate(&kernel_out, 0.0, p_j.seg_out_half_length, 8).unwrap();

    // --- TERMO VETORIAL (Z_vec) ---
    // Projeção do campo na tangente do nó do observador (s_i)
    let unit_i_avg = (p_i.seg_in_unit + p_i.seg_out_unit).normalize();
    let dot_in = p_j.seg_in_unit.dot(&unit_i_avg);
    let dot_out = p_j.seg_out_unit.dot(&unit_i_avg);

    // Z_vec = (j * omega * A . s_i) * L_i  -> O L_i converte V/m para Volts (Ohms)
    let z_vec = Complex64::new(0.0, k * ETA) * (dot_in * a_in + dot_out * a_out) * p_i.total_length;

    // --- TERMO ESCALAR (Z_sca) ---
    // Integrais de G para os centros dos segmentos do observador
    let psi_in_in = integrate(&GreenKernel { observation_point: p_i.seg_in_midpoint, ..kernel_in }, 0.0, p_j.seg_in_half_length, 8).unwrap();
    let psi_in_out = integrate(&GreenKernel { observation_point: p_i.seg_in_midpoint, ..kernel_out }, 0.0, p_j.seg_out_half_length, 8).unwrap();
    let psi_out_in = integrate(&GreenKernel { observation_point: p_i.seg_out_midpoint, ..kernel_in }, 0.0, p_j.seg_in_half_length, 8).unwrap();
    let psi_out_out = integrate(&GreenKernel { observation_point: p_i.seg_out_midpoint, ..kernel_out }, 0.0, p_j.seg_out_half_length, 8).unwrap();

    // Phi(r) = 1/(j*w*eps) * [ (1/L_out)*Int_G_out - (1/L_in)*Int_G_in ]
    let phi_at_mi_out = (psi_out_out / p_j.seg_out_half_length) - (psi_out_in / p_j.seg_in_half_length);
    let phi_at_mi_in = (psi_in_out / p_j.seg_out_half_length) - (psi_in_in / p_j.seg_in_half_length);

    // Z_sca = Phi_out - Phi_in  (Diferença de potencial já sai em Volts/Ampere = Ohms)
    let z_scalar = Complex64::new(0.0, -ETA / k) * (phi_at_mi_out - phi_at_mi_in);

    z_vec  + z_scalar
}
// fn calculate_pulse_interaction_old(p_i: &Pulse, p_j: &Pulse, k: f64) -> Complex64 {
//     // Vector potential contribution
//
//     let kernel_in = GreenKernel {
//         observation_point: p_i.center_node,
//         source_start: p_j.seg_in_midpoint,
//         source_unit_vector: p_j.seg_in_unit,
//         radius: p_j.seg_in_radius,
//         k,
//     };
//
//     let kernel_out = GreenKernel {
//         observation_point: p_i.center_node,
//         source_start: p_j.center_node,
//         source_unit_vector: p_j.seg_out_unit,
//         radius: p_j.seg_out_radius,
//         k,
//     };
//
//     let a_in = integrate(&kernel_in, 0.0, p_j.seg_in_half_length, 8).unwrap();
//     let a_out = integrate(&kernel_out, 0.0, p_j.seg_out_half_length, 8).unwrap();
//
//     let unit_i_avg = (p_i.seg_in_unit + p_i.seg_out_unit).normalize(); // Average unit vector for pulse i (point matching)
//
//     let z_vec = Complex64::new(0.0, k * ETA)
//         * ((a_in * p_j.seg_in_unit.dot(&unit_i_avg) * p_j.seg_in_half_length)
//             + (a_out * p_j.seg_out_unit.dot(&unit_i_avg) * p_j.seg_out_half_length));
//
//     // Scalar potential contribution (quasi-static approximation)
//
//     // The scalar potential is the difference of Phi at the centers of the observer's segments
//     // due to the charges at the centers of the source's segments.
//     let psi_in_in = green_function(
//         p_i.seg_in_midpoint,
//         p_j.seg_in_midpoint,
//         p_j.seg_in_radius,
//         k,
//     );
//     let psi_in_out = green_function(
//         p_i.seg_in_midpoint,
//         p_j.seg_out_midpoint,
//         p_j.seg_out_radius,
//         k,
//     );
//     let psi_out_in = green_function(
//         p_i.seg_out_midpoint,
//         p_j.seg_in_midpoint,
//         p_j.seg_in_radius,
//         k,
//     );
//     let psi_out_out = green_function(
//         p_i.seg_out_midpoint,
//         p_j.seg_out_midpoint,
//         p_j.seg_out_radius,
//         k,
//     );
//
//     // In MiniNEC, the charge is proportional to 1/length of each arm
//     let z_scalar =
//         (Complex64::new(0.0, -ETA / k)) * ((psi_out_out - psi_out_in) - (psi_in_out - psi_in_in));
//
//     z_vec + z_scalar
// }

fn green_function(
    p_i_midpoint: Point3<f64>,
    p_j_midpoint: Point3<f64>,
    p_j_radius: f64,
    k: f64,
) -> Complex64 {
    let kernel = GreenKernel {
        observation_point: p_i_midpoint,
        source_start: p_j_midpoint,
        source_unit_vector: Vector3::zeros(),
        radius: p_j_radius,
        k,
    };
    kernel.eval(0.0)
}
