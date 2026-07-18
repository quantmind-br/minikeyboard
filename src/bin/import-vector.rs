//! Convert reviewed USB capture payloads into golden test vectors.
//!
//! Fail-closed on source metadata: if a JSON capture metadata file provides
//! bus/device, `--bus`/`--device` are required and must match.
//! Does **not** elevate protocol capability — only writes fixtures under tests/vectors/.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};

fn usage() {
    eprintln!(
        "usage: import-vector --tuple VID-PID --operation NAME \\\n\
         \t--request request.bin [--response response.bin] \\\n\
         \t--out tests/vectors [--source CAPTURE] [--bus N] [--device M]\n\n\
         Writes request.bin/response.bin/expected.json under\n\
         <out>/<tuple>/<operation>/ with SHA-256 and provenance metadata.\n\
         If --source points at JSON metadata with bus/device, --bus and --device\n\
         are required and must match (unrelated USB addresses are rejected)."
    );
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.windows(2)
        .find(|w| w[0] == name)
        .map(|w| w[1].clone())
}

fn sha256_hex(data: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(data);
    hex::encode(h.finalize())
}

/// Fail-closed provenance check when `--source` is a JSON metadata file.
fn reject_unrelated_usb(
    source: &Option<String>,
    bus: &Option<String>,
    device: &Option<String>,
) -> Result<(), ExitCode> {
    let Some(src) = source else {
        return Ok(());
    };
    let src_path = Path::new(src);
    if src_path.extension().and_then(|e| e.to_str()) != Some("json") {
        return Ok(());
    }

    let text = fs::read_to_string(src_path).map_err(|e| {
        eprintln!("error: cannot read source metadata {src}: {e}");
        ExitCode::from(1)
    })?;
    let meta: serde_json::Value = serde_json::from_str(&text).map_err(|e| {
        eprintln!("error: cannot parse source metadata {src}: {e}");
        ExitCode::from(1)
    })?;

    let has_bus = meta.get("bus").is_some();
    let has_device = meta.get("device").is_some();
    if !has_bus && !has_device {
        return Ok(());
    }
    if !has_bus || !has_device {
        eprintln!("error: source metadata must include both bus and device when either is present");
        return Err(ExitCode::from(1));
    }

    let (Some(want_b), Some(want_d)) = (bus, device) else {
        eprintln!(
            "error: source metadata includes bus/device — pass matching --bus and --device"
        );
        return Err(ExitCode::from(1));
    };

    let mb = meta.get("bus").unwrap().to_string().trim_matches('"').to_string();
    let md = meta
        .get("device")
        .unwrap()
        .to_string()
        .trim_matches('"')
        .to_string();
    if mb != *want_b || md != *want_d {
        eprintln!(
            "error: capture metadata bus/device ({mb}/{md}) != --bus/--device ({want_b}/{want_d}) — refusing unrelated USB address"
        );
        return Err(ExitCode::from(1));
    }
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        usage();
        return if args.is_empty() {
            ExitCode::from(2)
        } else {
            ExitCode::SUCCESS
        };
    }

    let Some(tuple) = arg_value(&args, "--tuple") else {
        eprintln!("error: --tuple required (e.g. 1189-8842)");
        return ExitCode::from(2);
    };
    let Some(operation) = arg_value(&args, "--operation") else {
        eprintln!("error: --operation required");
        return ExitCode::from(2);
    };
    let Some(request_path) = arg_value(&args, "--request").map(PathBuf::from) else {
        eprintln!("error: --request required");
        return ExitCode::from(2);
    };
    let response_path = arg_value(&args, "--response").map(PathBuf::from);
    let out_root =
        PathBuf::from(arg_value(&args, "--out").unwrap_or_else(|| "tests/vectors".into()));
    let source = arg_value(&args, "--source");
    let bus = arg_value(&args, "--bus");
    let device = arg_value(&args, "--device");

    if let Err(code) = reject_unrelated_usb(&source, &bus, &device) {
        return code;
    }

    let request = match fs::read(&request_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("error: read request: {e}");
            return ExitCode::from(1);
        }
    };
    if request.len() != 65 {
        eprintln!(
            "error: request must be exactly 65 bytes, got {}",
            request.len()
        );
        return ExitCode::from(1);
    }

    let response = match response_path {
        Some(p) => match fs::read(p) {
            Ok(b) => Some(b),
            Err(e) => {
                eprintln!("error: read response: {e}");
                return ExitCode::from(1);
            }
        },
        None => None,
    };

    let dest = out_root.join(&tuple).join(&operation);
    if let Err(e) = fs::create_dir_all(&dest) {
        eprintln!("error: create {dest:?}: {e}");
        return ExitCode::from(1);
    }

    if let Err(e) = fs::write(dest.join("request.bin"), &request) {
        eprintln!("error: write request.bin: {e}");
        return ExitCode::from(1);
    }
    if let Some(resp) = &response
        && let Err(e) = fs::write(dest.join("response.bin"), resp)
    {
        eprintln!("error: write response.bin: {e}");
        return ExitCode::from(1);
    }

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let provenance = if response.is_some() {
        "captured-request-response"
    } else {
        "documented-request"
    };

    let expected = serde_json::json!({
        "command": operation,
        "hardware_tuple": tuple,
        "timestamp_unix": ts,
        "request_sha256": sha256_hex(&request),
        "response_sha256": response.as_ref().map(|r| sha256_hex(r)),
        "source_capture": source,
        "bus": bus,
        "device": device,
        "provenance": provenance,
        "notes": "Imported by import-vector. Review before elevating ReadCapability / verified_actions."
    });

    match fs::write(
        dest.join("expected.json"),
        serde_json::to_string_pretty(&expected).unwrap() + "\n",
    ) {
        Ok(()) => {
            println!("wrote {}", dest.display());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: write expected.json: {e}");
            ExitCode::from(1)
        }
    }
}
