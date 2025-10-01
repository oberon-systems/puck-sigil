use serial_test::serial;

#[test]
#[serial]
fn test_tag_format() {
    let version = "1.2.3";
    let tag = format!("v{}", version);
    assert_eq!(tag, "v1.2.3");
}
