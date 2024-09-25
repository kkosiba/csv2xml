use std::{fs::File, io::Read};

use chardetng::EncodingDetector;
use log::debug;

const SUPPORTED_ENCODINGS: [&str; 13] = [
    "windows-1252",
    "iso-8859-1",
    "iso-8859-2",
    "mac_roman",
    "mac_latin2",
    "utf-8",
    "utf-8-sig",
    "utf-16",
    "utf-16be",
    "utf-16le",
    "utf-32",
    "utf-32be",
    "utf-32le",
];

const MAX_BYTES_TO_READ: usize = 1 * 1024 * 1024; // 1 MB

/// Helper function to check if the encoding is supported
fn is_supported_encoding(encoding: &str) -> bool {
    SUPPORTED_ENCODINGS
        .iter()
        .any(|&supported| supported.eq_ignore_ascii_case(encoding))
}

pub fn check_encoding(file_path: &str) -> Result<(), anyhow::Error> {
    // Open the file
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => anyhow::bail!("Failed to open file '{:?}'. Error: {}", file_path, error),
    };
    // Read a chunk of the file
    let mut buffer = vec![0u8; MAX_BYTES_TO_READ];
    let mut handle = file.take(MAX_BYTES_TO_READ as u64);
    if let Err(error) = handle.read(&mut buffer) {
        anyhow::bail!(
            "Failed to read '{}' bytes from file '{:?}'. Error: {}",
            MAX_BYTES_TO_READ,
            file_path,
            error,
        )
    };

    // Detect the input file encoding
    let mut detector = EncodingDetector::new();
    detector.feed(&buffer, true);
    let detected_encoding = detector.guess(None, true).name();

    debug!("Detected encoding: {}", detected_encoding);

    // Check if the detected encoding is supported
    if !is_supported_encoding(detected_encoding) {
        anyhow::bail!(
            "Unsupported encoding: '{}'. Supported encodings are: {:?}",
            detected_encoding,
            SUPPORTED_ENCODINGS
        );
    }
    debug!("Encoding '{}' is supported.", detected_encoding);
    Ok(())
}
