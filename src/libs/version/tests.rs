use super::*;
use serial_test::serial;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
#[serial]
fn test_read_json() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("version.json");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, r#"{{"version": "1.2.3", "build": 456}}"#).unwrap();

    let result = read_version(file_path.to_str().unwrap(), "version");
    assert_eq!(result.unwrap(), "1.2.3");
}

#[test]
#[serial]
fn test_read_toml() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("version.toml");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "version = \"2.0.0\"").unwrap();
    writeln!(file, "build = 789").unwrap();

    let result = read_version(file_path.to_str().unwrap(), "version");
    assert_eq!(result.unwrap(), "2.0.0");
}

#[test]
#[serial]
fn test_read_yaml() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("version.yaml");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "version: 3.0.0").unwrap();
    writeln!(file, "build: 123").unwrap();

    let result = read_version(file_path.to_str().unwrap(), "version");
    assert_eq!(result.unwrap(), "3.0.0");
}

#[test]
#[serial]
fn test_nested_json() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("package.json");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, r#"{{"package": {{"version": "4.5.6"}}}}"#).unwrap();

    let result = read_version(file_path.to_str().unwrap(), "package.version");
    assert_eq!(result.unwrap(), "4.5.6");
}

#[test]
#[serial]
fn test_unsupported_format() {
    let result = read_version("test.txt", "version");
    assert!(matches!(result, Err(VersionError::UnsupportedFormat(_))));
}
