use calcli::history_io::{export_history, import_history};
use std::fs;

#[test]
fn test_export_import_roundtrip() {
    let path = "/tmp/calcli_test_history.json";
    let entries = vec![
        ("2+2".to_string(), 4.0),
        ("3*5".to_string(), 15.0),
        ("sqrt(9)".to_string(), 3.0),
    ];

    export_history(path, &entries).expect("export should succeed");

    let imported = import_history(path).expect("import should succeed");
    assert_eq!(imported.len(), 3);
    assert_eq!(imported[0].expression, "2+2");
    assert_eq!(imported[0].result, 4.0);
    assert_eq!(imported[1].expression, "3*5");
    assert_eq!(imported[1].result, 15.0);
    assert_eq!(imported[2].expression, "sqrt(9)");
    assert_eq!(imported[2].result, 3.0);

    fs::remove_file(path).ok();
}

#[test]
fn test_export_empty_history() {
    let path = "/tmp/calcli_test_empty.json";
    export_history(path, &[]).expect("export of empty history should succeed");

    let imported = import_history(path).expect("import should succeed");
    assert!(imported.is_empty());

    fs::remove_file(path).ok();
}

#[test]
fn test_import_nonexistent_file() {
    let result = import_history("/tmp/calcli_nonexistent_file.json");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("File not found"));
}
