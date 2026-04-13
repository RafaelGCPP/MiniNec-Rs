#![allow(dead_code)]
use nalgebra::Point3;
use serde::Deserialize;
use std::fs;
use thiserror::Error;

/// Represents a single wire element in the antenna geometry.
#[derive(Deserialize, Debug)]
pub(super) struct Wire {
    /// Unique identifier for the wire.
    pub(super) id: String,
    /// Start point of the wire in 3D space.
    pub(super) start: Point3<f64>,
    /// End point of the wire in 3D space.
    pub(super) end: Point3<f64>,
    /// Diameter of the wire in meters.
    pub(super) diameter: f64,
}

/// Supported Ground Types.

#[derive(Deserialize, Debug)]
pub(super) enum GroundType {
    /// Free Space or no ground
    FreeSpace,
    /// Perfect Ground
    PerfectGround,
    // /// MiniNEC Ground model
    // MiniNec,
}

/// Represents the antenna geometry and simulation parameters loaded from a JSON file.
#[derive(Deserialize, Debug)]
pub(super) struct AntennaFile {
    /// List of wires that make up the antenna.
    pub(super) wires: Vec<Wire>,
    /// Ground type or model (e.g., "free_space", "perfect_ground").
    pub(super) ground: GroundType,
    /// Simulation frequency in Hz.
    pub(super) frequency: f64,
    /// Additional height above ground in meters.
    pub(super) added_height: f64,
}

/// Errors that can occur when reading or parsing an antenna geometry file.
#[derive(Error, Debug)]
pub enum AntennaFileError {
    /// File read error (e.g., file not found, permission denied).
    #[error("Erro de leitura de arquivo: {0}")]
    Io(#[from] std::io::Error),

    /// JSON parse error (invalid or malformed file contents).
    #[error("Erro de parse JSON: {0}")]
    Json(#[from] serde_json::Error),
}

/// Reads and parses an antenna geometry description from a JSON file.
///
/// # Parameters
/// - `filename`: Path to the JSON file containing the antenna geometry.
///
/// # Returns
/// Returns `Ok(AntennaFile)` if the file is successfully read and parsed.
/// Returns `Err(AntennaFileError)` if there is an I/O or JSON parsing error.
///
/// # Errors
/// - `AntennaFileError::Io` if the file cannot be read (e.g., not found).
/// - `AntennaFileError::Json` if the file contents cannot be parsed as valid JSON.
pub fn read_antenna_from_file(filename: &str) -> Result<AntennaFile, AntennaFileError> {
    let contents = fs::read_to_string(filename)?;
    let antenna = serde_json::from_str(&contents)?;

    Ok(antenna)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Checks that reading a non-existant file returns an Io(NotFound) error.
    fn antenna_file_not_exist() {
        let result = read_antenna_from_file("TestData/xantenna.json");

        assert!(
            matches!(result, Err(AntennaFileError::Io(ref e)) if e.kind() == std::io::ErrorKind::NotFound),
            "Should be Io(NotFound), but found: {:?}",
            result
        );
    }

    #[test]
    // Checks that a valid file is parsed as data
    fn antenna_file_ok() {
        let result = read_antenna_from_file("TestData/antenna.json");
        assert!(result.is_ok(), "Should be Ok, but found: {:?}", result);
    }
    #[test]
    // Checks that parsing errors are reported as Json
    fn antenna_file_json_error() {
        let result = read_antenna_from_file("TestData/error.json");
        assert!(
            matches!(result, Err(AntennaFileError::Json(_))),
            "Should be Json(), but found: {:?}",
            result
        );
    }
}
