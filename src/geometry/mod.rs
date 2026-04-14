mod geometry_file;
mod segments_compiler;
mod pulse_compiler;
mod model;

use model::*;
use thiserror::Error;

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