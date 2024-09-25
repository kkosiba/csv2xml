use assert_cmd::Command;
use std::error::Error;
use std::path::PathBuf;
use std::{fs, panic};

fn get_test_file_path(file_name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("data");
    path.push(file_name);
    path
}

fn get_command(
    input_path: &PathBuf,
    output_path: &PathBuf,
    delimiter: &str,
    encoding: &str,
) -> Result<Command, Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("csv2xml").unwrap();
    cmd.arg("--input")
        .arg(input_path.to_str().unwrap())
        .arg("--output")
        .arg(output_path.to_str().unwrap())
        .arg("--delimiter")
        .arg(delimiter.to_string())
        .arg("--encoding")
        .arg(encoding.to_string());
    Ok(cmd)
}

#[test]
fn test_main_utf8_file() {
    let input_path = get_test_file_path("utf8.csv");
    let output_path = get_test_file_path("output_utf8.xml");
    let encoding = "utf-8";
    let expected_xml = "<root><row><field0>Alice</field0><field1>30</field1></row>\
                        <row><field0>Bob</field0><field1>25</field1></row></root>";

    // Catch any panic that occurs during execution, to prevent early test abortion (before the
    // cleanup takes place)
    let _ = panic::catch_unwind(|| {
        let cmd = get_command(&input_path, &output_path, &",", &encoding);

        cmd.unwrap().assert().success();

        let output_data = fs::read_to_string(&output_path).unwrap();
        assert_eq!(output_data, expected_xml);
    });

    // Cleanup
    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_main_iso_8859_1_file() {
    let input_path = get_test_file_path("iso_8859_1.csv");
    let output_path = get_test_file_path("output_iso_8859_1.xml");
    let encoding = "iso-8859-1";
    let expected_xml = "<root><row><field0>Alice</field0><field1>30</field1></row>\
                        <row><field0>Bob</field0><field1>25</field1></row></root>";

    let _ = panic::catch_unwind(|| {
        let cmd = get_command(&input_path, &output_path, &",", &encoding);

        cmd.unwrap().assert().success();

        let output_data = fs::read_to_string(&output_path).unwrap();
        assert_eq!(output_data, expected_xml);
    });

    // Cleanup
    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_main_tab_delimited_file() {
    let input_path = get_test_file_path("tab_delimited.csv");
    let output_path = get_test_file_path("output_tab.xml");
    let encoding = "utf-8";
    let expected_xml = "<root><row><field0>Alice</field0><field1>30</field0></row>\
                        <row><field0>Bob</field0><field1>25</field1></row></root>";

    let _ = panic::catch_unwind(|| {
        let cmd = get_command(&input_path, &output_path, &"\t", &encoding);

        cmd.unwrap().assert().success();

        let output_data = fs::read_to_string(&output_path).unwrap();
        assert_eq!(output_data, expected_xml);
    });

    // Cleanup
    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_main_empty_file() {
    let input_path = get_test_file_path("empty.csv");
    let output_path = get_test_file_path("output_empty.xml");
    let encoding = "utf-8";
    let expected_xml = "<root></root>";

    let _ = panic::catch_unwind(|| {
        let cmd = get_command(&input_path, &output_path, &",", &encoding);

        cmd.unwrap().assert().success();

        let output_data = fs::read_to_string(&output_path).unwrap();
        assert_eq!(output_data, expected_xml);
    });

    // Cleanup
    fs::remove_file(output_path).unwrap();
}
