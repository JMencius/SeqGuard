use rayon::prelude::*;
use std::{collections::{HashSet, HashMap}, fs::File, io::{BufRead, BufReader}, sync::{Arc, Mutex}};
use flate2::read::GzDecoder;
use clap::Parser;

mod cli;
use cli::Args; 


fn main() {
    let args = Args::parse();

    let reader = match File::open(&args.input) {
        Ok(file) => {
            if args.input.ends_with(".gz") {
                Box::new(BufReader::new(GzDecoder::new(file))) as Box<dyn BufRead>
            } else {
                Box::new(BufReader::new(file)) as Box<dyn BufRead>
            }
        }
        Err(e) => {
            println!("QC Result: FAIL");
            eprintln!("Failed to open file: {}", e);
            return;
        }
    };

    let lines: Vec<String> = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|l| !l.trim().is_empty()) // Skip empty lines
        .collect();

    if lines.is_empty() {
        println!("QC Result: FAIL");
        eprintln!("File is empty or contains only blank lines.");
        return;
    }

    if lines.len() % 4 != 0 {
        eprintln!("QC Result: FAIL");
        eprintln!("FASTQ format error: number of lines ({}) is not a multiple of 4", lines.len());
        return;
    }

    let total_records = lines.len() / 4;
    let lines = Arc::new(lines);
    let headers_seen = Arc::new(Mutex::new(HashSet::new()));
    let non_atgc_counts = Arc::new(Mutex::new(HashMap::new()));

    // Use Rayon for parallel processing
    let qc_results: Vec<bool> = (0..total_records).into_par_iter().map(|i| {
        let header = &lines[i * 4];
        let seq = &lines[i * 4 + 1];
        let qual = &lines[i * 4 + 3];

        let mut qc_pass = true;

        // Check header
        qc_pass &= check_header(i, header);

        // Check duplicate header
        qc_pass &= check_duplicate_header(i, header, &headers_seen);

        // Check sequence and quality length
        qc_pass &= check_sequence_length(i, seq, qual);

        // Check non-ATGC bases
        qc_pass &= check_non_atgc(i, seq, &non_atgc_counts);

        // Check quality value range
        qc_pass &= check_quality_value(i, qual);

        qc_pass
    }).collect();

    // Determine if any record failed
    let qc_pass = qc_results.iter().all(|&result| result);

    // Output QC result (first line)
    if qc_pass {
        println!("QC Result: PASS");
    } else {
        println!("QC Result: FAIL");
    }

    // Non-ATGC base report (if any)
    let non_atgc = non_atgc_counts.lock().unwrap();
    if !non_atgc.is_empty() {
        println!("\nNon-ATGC base report:");
        for (base, count) in non_atgc.iter() {
            println!("  {}: {}", base, count);
        }
    } else {
        println!("\nAll bases are A, T, G, or C.");
    }
}

// Function to check header
fn check_header(record_index: usize, header: &str) -> bool {
    if !(header.starts_with('@') || header.starts_with('>')) {
        eprintln!("Invalid header line (record {}): {}", record_index + 1, header);
        false
    } else {
        true
    }
}

// Function to check duplicate headers
fn check_duplicate_header(record_index: usize, header: &str, headers_seen: &Arc<Mutex<HashSet<String>>>) -> bool {
    let mut seen = headers_seen.lock().unwrap();
    if !seen.insert(header.to_string()) {
        eprintln!("Duplicate header found (record {}): {}", record_index + 1, header);
        false
    } else {
        true
    }
}

// Function to check sequence and quality length match
fn check_sequence_length(record_index: usize, seq: &str, qual: &str) -> bool {
    if seq.len() != qual.len() {
        eprintln!("Length mismatch (record {}): seq = {}, qual = {}", record_index + 1, seq.len(), qual.len());
        false
    } else {
        true
    }
}

// Function to check for non-ATGC bases
fn check_non_atgc(_record_index: usize, seq: &str, non_atgc_counts: &Arc<Mutex<HashMap<char, usize>>>) -> bool {
    let mut counts = non_atgc_counts.lock().unwrap();
    for base in seq.chars() {
        let upper = base.to_ascii_uppercase();
        if !"ATGC".contains(upper) {
            *counts.entry(upper).or_insert(0) += 1;
        }
    }
    true
}

// Function to check quality value range (0 to 93)
fn check_quality_value(record_index: usize, qual: &str) -> bool {
    let mut valid = true;
    for (j, c) in qual.chars().enumerate() {
        let ascii_value = c as u8;
        // check if qv char within range (ASCII 33-126)
        if ascii_value < 33 || ascii_value > 126 {
            eprintln!("Invalid character in quality string (record {}, position {}): '{}'", record_index + 1, j + 1, c);
            valid = false;
            continue; // skip
        }
    }
    valid
}

