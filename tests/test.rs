use std::process::Command;
use std::fs;



fn run_seqguard_on(file: &str) -> String {
    let output = Command::new("./target/debug/seqguard")
        .args(["-i", file, "-t", "8"])
        .output()
        .expect("Failed to run seqguard");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr)
}



#[test]
fn test_normal_fastq() {
    let result = run_seqguard_on("tests/normal.fastq");
    println!("{}", result);
    assert!(result.contains("QC Result: PASS"));
}


#[test]
fn test_duplicate_header() {
    let result = run_seqguard_on("tests/duplicate_header.fastq");
    println!("{}", result);
    assert!(result.contains("QC Result: FAIL"));
    assert!(result.contains("Duplicate header"));
}


#[test]
fn test_invalid_format() {
    let result = run_seqguard_on("tests/invalid_format.fastq");
    println!("{}", result);
    assert!(result.contains("QC Result: FAIL"));
    assert!(result.contains("FASTQ format error"));
}


#[test]
fn test_invalid_qv() {
    let result = run_seqguard_on("tests/invalid_qv.fastq");
    println!("{}", result);
    assert!(result.contains("QC Result: FAIL"));
    assert!(result.contains("Invalid character in quality string"))
}


#[test]
fn test_mismatch_len() {
    let result = run_seqguard_on("tests/mismatch_len.fastq");
    println!("{}", result);
    assert!(result.contains("QC Result: FAIL"));
    assert!(result.contains("Length mismatch"))
}


#[test]
fn test_non_atgc() {
    let result = run_seqguard_on("tests/non_atgc.fastq");
    println!("{}", result);
    assert!(result.contains("QC Result: PASS"));
    assert!(result.contains("Non-ATGC base"))
}


#[test]
fn test_empty_fastq() {
    let result = run_seqguard_on("tests/empty.fastq");
    println!("{}", result);
    assert!(result.contains("QC Result: FAIL"));
    assert!(result.contains("File is empty or contains only blank lines"))
}



