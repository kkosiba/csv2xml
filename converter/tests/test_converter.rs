use converter::convert;
use pretty_assertions::assert_eq;
use tempfile::NamedTempFile;

fn normalize(s: &str) -> String {
    s.lines()
        .map(|line| line.trim_end()) // Trim trailing whitespace from each line
        .collect::<Vec<_>>()
        .join("\n") // Join with '\n' to ensure consistent line endings
}

macro_rules! run_test {
    ($func:ident, $input_file:expr, $expected_output_file:expr) => {
        #[test]
        fn $func() -> Result<(), Box<dyn std::error::Error>> {
            let input_file_path = format!("./tests/data/inputs/{}", $input_file);
            std::fs::exists(&input_file_path)
                .expect(&format!("Input file '{}' does not exist", &input_file_path));

            let expected_output_file_path =
                format!("./tests/data/expected_outputs/{}", $expected_output_file);

            let actual_output_file = NamedTempFile::new().expect("Failed to create a temp file");
            let actual_output_file_path = actual_output_file.path().to_str().unwrap();

            convert(&input_file_path, actual_output_file_path, '\t').expect("Conversion failed");

            let expected_output_contents = std::fs::read_to_string(&expected_output_file_path)
                .expect(&format!(
                    "Failed to read expected output file '{}'",
                    &expected_output_file_path
                ));
            let actual_output_contents =
                std::fs::read_to_string(&actual_output_file_path).expect(&format!(
                    "Failed to read actual output file '{}'",
                    &actual_output_file_path
                ));
            assert_eq!(
                normalize(&actual_output_contents),
                normalize(&expected_output_contents)
            );
            Ok(())
        }
    };
}

run_test!(
    convert_with_empty_lines,
    "empty_lines.tsv",
    "empty_lines.xml"
);

run_test!(
    convert_with_extra_columns,
    "extra_columns.tsv",
    "extra_columns.xml"
);

run_test!(
    convert_with_missing_columns,
    "missing_columns.tsv",
    "missing_columns.xml"
);
