use assert_cmd::Command;
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
fn test_main() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data");
    let test_cases_path = data_dir.join("test_cases.json");
    let test_cases: Vec<TestCase> = serde_json::from_str(&fs::read_to_string(&test_cases_path)?)?;

    let temp_dir = tempdir()?;
    for (index, test_case) in test_cases.iter().enumerate() {
        let output_file = temp_dir.path().join(format!("output_{}.xml", index));
        File::create(output_file.as_path())?;

        let mut cmd = Command::cargo_bin("csv2xml")?;
        let input_file_full_path = data_dir.join(&test_case.input_file);
        if !input_file_full_path.exists() {
            return Err(format!("File {:?} does not exist", input_file_full_path).into());
        }
        cmd.arg(input_file_full_path);
        cmd.arg(&output_file);
        if let Some(args) = &test_case.extra_cli_args {
            cmd.args(args);
        }

        let expected_output_contents =
            fs::read_to_string(data_dir.join(&test_case.expected_output_file))?;
        let actual_output_contents = fs::read_to_string(&output_file)?;

        cmd.assert().success();
        assert_eq!(
            expected_output_contents.trim(),
            actual_output_contents.trim(),
            "Mismatch in test case: {}",
            test_case.description
        );
    }
    Ok(())
}
