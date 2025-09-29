use super::*;
use serial_test::serial;
use std::env;

#[test]
#[serial]
fn test_parse_args_default() {
    unsafe {
        env::remove_var("PSIGIL_CONFIG");
    }

    let args = parse_args();
    assert_eq!(args.config_path, "config.json");
}

#[test]
#[serial]
fn test_parse_args_from_env() {
    unsafe {
        env::set_var("PSIGIL_CONFIG", "/test/path.json");
    }

    let args = parse_args();
    assert_eq!(args.config_path, "/test/path.json");

    unsafe {
        env::remove_var("PSIGIL_CONFIG");
    }
}
