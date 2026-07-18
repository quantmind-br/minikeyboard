//! Golden vector loading — distinguish documented requests from captured responses.

use std::fs;
use std::path::PathBuf;

use minikeyboard::protocol::frame::{commit_report, identify_report, read_config_report};

fn vectors_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/vectors")
}

#[test]
fn identify_request_matches_constructor() {
    let path = vectors_root().join("1189-8842/identify/request.bin");
    let bytes = fs::read(&path).expect("request.bin");
    assert_eq!(bytes.len(), 65);
    assert_eq!(bytes.as_slice(), identify_report().as_slice());

    let meta_path = vectors_root().join("1189-8842/identify/expected.json");
    let meta: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(meta_path).unwrap()).unwrap();
    assert_eq!(meta["provenance"], "documented-request");
    // Documented request alone must not imply a captured response.
    assert!(!vectors_root()
        .join("1189-8842/identify/response.bin")
        .exists());
}

#[test]
fn commit_and_read_frames_stable() {
    assert_eq!(&commit_report()[..4], &[0x03, 0xFD, 0xFE, 0xFF]);
    assert_eq!(&read_config_report(3, 15, 3)[..5], &[0x03, 0xFA, 3, 15, 3]);
}

#[test]
fn captured_read_capability_is_verified() {
    let cap = minikeyboard::protocol::read_config::current_read_capability();
    assert!(matches!(
        cap,
        minikeyboard::protocol::read_config::ReadCapability::Verified { .. }
    ));
}
