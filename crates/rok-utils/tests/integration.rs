use rok_utils::{fs, path, string};

#[test]
fn write_and_read() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("out.txt");
    fs::write_atomic(&file, b"rok").unwrap();
    assert_eq!(fs::read_to_string(&file).unwrap(), "rok");
}

#[test]
fn path_normalize() {
    assert_eq!(
        path::normalize("a/b/../c/./d"),
        std::path::PathBuf::from("a/c/d")
    );
}

#[test]
fn string_conversions() {
    assert_eq!(string::to_camel_case("hello_world"), "helloWorld");
    assert_eq!(string::to_pascal_case("hello_world"), "HelloWorld");
    assert_eq!(string::to_snake_case("helloWorld"), "hello_world");
    assert_eq!(string::to_kebab_case("HelloWorld"), "hello-world");
    assert_eq!(string::to_screaming_snake("helloWorld"), "HELLO_WORLD");
}
