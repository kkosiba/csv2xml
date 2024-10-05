use converter::convert;
use pretty_assertions::assert_eq;
use quick_xml::reader::Reader;
use serde::Deserialize;
use std::fs::{self, File};
use std::path::PathBuf;
use tempfile::tempdir;

#[derive(Deserialize)]
struct TestCase {
    description: String,
    input_file: String,
    expected_output_file: String,
    extra_cli_args: Option<Vec<String>>,
}

#[test]
fn test_converter() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = PathBuf::from(".").join("tests").join("data");
    let test_cases_path = data_dir.join("test_cases.json");
    let test_cases: Vec<TestCase> = serde_json::from_str(&fs::read_to_string(&test_cases_path)?)?;

    let temp_dir = tempdir()?;
    for (index, test_case) in test_cases.iter().enumerate() {
        let output_file = temp_dir.path().join(format!("output_{}.xml", index));
        File::create(output_file.as_path())?;

        let input_file_full_path = data_dir.join(&test_case.input_file);
        if !input_file_full_path.exists() {
            return Err(format!("File {:?} does not exist", input_file_full_path).into());
        }
        if let Some(args) = &test_case.extra_cli_args {
            todo!("Pass extra {args:?} to converter");
        }
        convert(
            input_file_full_path.to_str().unwrap(),
            output_file.to_str().unwrap(),
            '\t',
        )?;

        let expected_output_contents =
            fs::read_to_string(data_dir.join(&test_case.expected_output_file))?;
        let expected_xml = Reader::from_str(&expected_output_contents.trim());
        let actual_output_contents = fs::read_to_string(&output_file)?;
        let actual_xml = Reader::from_str(&actual_output_contents.trim());

        assert_eq!(
            std::str::from_utf8(expected_xml.into_inner()),
            std::str::from_utf8(actual_xml.into_inner()),
            "Mismatch in test case: {}",
            test_case.description
        );
    }
    Ok(())
}
