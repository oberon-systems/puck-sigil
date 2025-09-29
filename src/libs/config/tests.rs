use super::*;
use serial_test::serial;
use std::fs;

#[test]
#[serial]
fn test_load_config_success() {
    let json_content = r#"
    {
        "version_file": "version.txt",
        "version_param": "--version"
    }
    "#;

    fs::write("test_config.json", json_content).unwrap();

    let result = load_config("test_config.json");

    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.version_file, "version.txt");
    assert_eq!(config.version_param, "--version");

    fs::remove_file("test_config.json").unwrap();
}

#[test]
#[serial]
fn test_load_config_file_not_found() {
    let result = load_config("nonexistent.json");
    assert!(result.is_err());
}

#[test]
#[serial]
fn test_load_config_invalid_json() {
    fs::write("invalid.json", "not a json").unwrap();

    let result = load_config("invalid.json");
    assert!(result.is_err());

    fs::remove_file("invalid.json").unwrap();
}
