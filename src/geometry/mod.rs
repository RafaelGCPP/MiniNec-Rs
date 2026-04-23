mod geometry_file;
pub mod model;
mod pulse_compiler;
mod segments_compiler;

use model::*;
use physical_constants::SPEED_OF_LIGHT_IN_VACUUM;
use std::f64::consts::PI;
use std::path::Path;
use thiserror::Error;

const C0: f64 = SPEED_OF_LIGHT_IN_VACUUM;

/// Errors that can occur when reading or parsing an antenna geometry file.
#[derive(Error, Debug)]
pub enum AntennaFileError {
    /// File read error (e.g., file not found, permission denied).
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON parse error (invalid or malformed file contents).
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Geometry Compilation Error: {0}")]
    Compile(String),
}

pub fn load_file(path: impl AsRef<Path>, segment_size_divider: f64) -> Result<Problem, AntennaFileError> {
    let antenna_file = geometry_file::read_antenna_from_file(path)?;
    let antenna = segments_compiler::compile_geometry_file(&antenna_file, segment_size_divider)?;
    let pulses = pulse_compiler::compile_pulses(&antenna)?;

    let frequency = antenna_file.frequency;
    let wave_number = frequency * 2.0 * PI / C0; // k = 2πf/c, where c is the speed of light in m/s

    Ok(Problem {
        frequency,
        wave_number,
        nodes: antenna.nodes,
        segments: antenna.segments,
        wire_map: antenna.wire_map,
        pulses,
    })
}
