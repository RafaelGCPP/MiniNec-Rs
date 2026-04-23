use super::*;
use std::fs;

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
pub fn read_antenna_from_file(path: impl AsRef<std::path::Path>) -> Result<AntennaFile, AntennaFileError> {
    let contents = fs::read_to_string(path)?;
    let antenna = serde_json::from_str(&contents)?;

    Ok(antenna)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Checks that reading a non-existent file returns an Io(NotFound) error.
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
