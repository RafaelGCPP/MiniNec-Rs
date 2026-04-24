mod integrate;
mod kernel;
mod system_builder;

pub use system_builder::{build_v_vector, build_z_matrix};

use physical_constants::CHARACTERISTIC_IMPEDANCE_OF_VACUUM;
pub const ETA: f64 = CHARACTERISTIC_IMPEDANCE_OF_VACUUM;
