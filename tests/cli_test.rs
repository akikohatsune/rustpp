use std::process::{Command, Stdio};
use std::io::Write;

const BIN: &str = env!("CARGO_BIN_EXE_oppai");
const SAMPLE_MAP: &str = "tests/fixtures/sample.osu";

#[test]
fn test_cli_version() {
    let out = Command::new(BIN).arg("-v").output().unwrap();
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "4.1.0");

    let out2 = Command::new(BIN).arg("-version").output().unwrap();
    assert!(out2.status.success());
    assert_eq!(String::from_utf8_lossy(&out2.stdout).trim(), "4.1.0");
}

#[test]
fn test_cli_usage_banner() {
    let out = Command::new(BIN).output().unwrap();
    assert!(!out.status.success()); // exits 1
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("usage:"));
    assert!(stderr.contains("set filename to '-'"));
}

#[test]
fn test_cli_modules_help() {
    let out = Command::new(BIN).args(["-", "-o?"]).output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("null"));
    assert!(stdout.contains("text"));
    assert!(stdout.contains("json"));
    assert!(stdout.contains("csv"));
    assert!(stdout.contains("binary"));
    assert!(stdout.contains("gnuplot"));
}

#[test]
fn test_cli_text_output() {
    let out = Command::new(BIN)
        .args([SAMPLE_MAP, "+HDDT", "98%", "400x", "1m"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Nogizaka46 - Yubi Bouenkyou"));
    assert!(stdout.contains("AR10.6 OD10.67"));
    assert!(stdout.contains("stars"));
    assert!(stdout.contains("speed strain:"));
    assert!(stdout.contains("aim strain:"));
    assert!(stdout.contains("+HDDT"));
    assert!(stdout.contains("pp"));
}

#[test]
fn test_cli_json_output() {
    let out = Command::new(BIN)
        .args([SAMPLE_MAP, "+HDHR", "99%", "-ojson"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.starts_with('{') && stdout.trim().ends_with('}'));
    assert!(stdout.contains("\"oppai_version\":\"4.1.0\""));
    assert!(stdout.contains("\"code\":200"));
    assert!(stdout.contains("\"errstr\":\"no error\""));
    assert!(stdout.contains("\"artist\":\"Nogizaka46\""));
    assert!(stdout.contains("\"artist_unicode\":"));
    assert!(stdout.contains("\"mods_str\":\"HDHR\""));
    assert!(stdout.contains("\"pp\":"));
}

#[test]
fn test_cli_csv_output() {
    let out = Command::new(BIN)
        .args([SAMPLE_MAP, "+HR", "-ocsv"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("oppai_version;4.1.0"));
    assert!(stdout.contains("code;200"));
    assert!(stdout.contains("artist;Nogizaka46"));
    assert!(stdout.contains("title;Yubi Bouenkyou (TV Size)"));
    assert!(stdout.contains("pp;"));
}

#[test]
fn test_cli_binary_output() {
    let out = Command::new(BIN)
        .args([SAMPLE_MAP, "+HD", "-obinary"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let bytes = &out.stdout;
    assert!(bytes.len() > 16);
    assert_eq!(&bytes[0..8], b"binoppai");
    assert_eq!(bytes[8], 4); // major
    assert_eq!(bytes[9], 1); // minor
    assert_eq!(bytes[10], 0); // patch
}

#[test]
fn test_cli_gnuplot_output() {
    let out = Command::new(BIN)
        .args([SAMPLE_MAP, "-ognuplot"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("set encoding utf8;"));
    assert!(stdout.contains("set multiplot layout 2,1 rowsfirst;"));
    assert!(stdout.contains("plot '-' with lines lc 1 title 'speed'"));
    assert!(stdout.contains("plot '-' with lines lc 2 title 'aim'"));
}

#[test]
fn test_cli_null_output() {
    let out = Command::new(BIN)
        .args([SAMPLE_MAP, "-onull"])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(out.stdout.is_empty());
}

#[test]
fn test_cli_stdin_piping() {
    let map_data = std::fs::read_to_string(SAMPLE_MAP).unwrap();
    let mut child = Command::new(BIN)
        .args(["-", "+HDDT", "98%"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(map_data.as_bytes()).unwrap();
    }

    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Nogizaka46"));
    assert!(stdout.contains("98."));
}

#[test]
fn test_cli_taiko_mode() {
    let out = Command::new(BIN)
        .args([SAMPLE_MAP, "-taiko", "-ojson"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("\"stars\":"));
    assert!(stdout.contains("\"pp\":"));
}

#[test]
fn test_cli_stat_overrides() {
    let out = Command::new(BIN)
        .args([SAMPLE_MAP, "AR10", "OD9", "CS5", "-ojson"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("\"ar\":10"));
    assert!(stdout.contains("\"od\":9"));
    assert!(stdout.contains("\"cs\":5"));
}

#[test]
fn test_cli_error_handling() {
    // Non-existent file
    let out = Command::new(BIN)
        .args(["non_existent_file_xyz.osu"])
        .output()
        .unwrap();
    assert!(!out.status.success());

    // JSON error response
    let out_json = Command::new(BIN)
        .args(["non_existent_file_xyz.osu", "-ojson"])
        .output()
        .unwrap();
    assert!(!out_json.status.success());
    let stdout = String::from_utf8_lossy(&out_json.stdout);
    assert!(stdout.contains("\"code\":-5")); // ERR_IO
    assert!(stdout.contains("\"errstr\":\"i/o error\""));
}
