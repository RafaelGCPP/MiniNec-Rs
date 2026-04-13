#![allow(dead_code)]
use thiserror::Error;
use serde::Deserialize;
use std::fs;
use nalgebra::Point3;

#[derive(Deserialize,Debug)]
struct Wire {
    start: Point3<f64>,
    end: Point3<f64>,
    diameter: f64
}

#[derive(Deserialize, Debug)]
pub struct Antenna {
    wires: Vec<Wire>,
    ground: String
}

#[derive(Error, Debug)]
pub enum AntennaFileError {
    #[error("Erro de leitura de arquivo: {0}")]
    Io(#[from] std::io::Error),

    #[error("Erro de parse JSON: {0}")]
    Json(#[from] serde_json::Error),
}

pub fn read_antenna_from_file(filename: &str) -> Result<Antenna, AntennaFileError> {
    let contents = fs::read_to_string(filename)?;
    let antenna= serde_json::from_str(&contents)?;

    Ok(antenna)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn antenna_file_not_exist() {
        let result = read_antenna_from_file("TestData/xantenna.json");

        assert!(
            matches!(result, Err(AntennaFileError::Io(ref e)) if e.kind() == std::io::ErrorKind::NotFound),
            "Should be Io(NotFound), but found: {:?}", result
        );
    }

    #[test]
    fn antenna_file_ok() {
        let result = read_antenna_from_file("TestData/antenna.json");
        assert!(result.is_ok(), "Should be Ok, but found: {:?}", result);
    }
    #[test]
    fn antenna_file_json_error() {
        let result = read_antenna_from_file("TestData/error.json");
        assert!(
            matches!(result, Err(AntennaFileError::Json(_)) ),
            "Should be Json(), but found: {:?}", result
        );
    }
}