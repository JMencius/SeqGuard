use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;
use flate2::read::GzDecoder;

mod cli;
use cli::Args;

fn main() {
    let args = Args::parse();

    let reader: Box<dyn BufRead> = match File::open(&args.input) {
        Ok(file) => {
            if args.input.ends_with(".gz") {
                Box::new(BufReader::new(GzDecoder::new(file)))
            } else {
                Box::new(BufReader::new(file))
            }
        }
        Err(e) => {
            println!("QC Result: FAIL");
            eprintln!("Failed to open file: {}", e);
            return;
        }
    };

    let mut line_buffer = Vec::with_capacity(4);
    let mut headers_seen = HashSet::new();
    let mut non_atgc_counts: HashMap<char, usize> = HashMap::new();
    let mut line_num = 0;

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l,
            Err(e) => {
                println!("QC Result: FAIL");
                eprintln!("Error reading line {}: {}", line_num + 1, e);
                return;
            }
        };

        line_num += 1;
        if line.trim().is_empty() {
            continue;
        }

        line_buffer.push(line);

        if line_buffer.len() == 4 {
            let header = &line_buffer[0];
            let seq = &line_buffer[1];
            let _plus = &line_buffer[2];
            let qual = &line_buffer[3];

            let record_index = (line_num / 4) - 1;

            if !check_header(record_index, header)
                || !check_duplicate_header(record_index, header, &mut headers_seen)
                || !check_sequence_length(record_index, seq, qual)
                || !check_non_atgc(seq, &mut non_atgc_counts)
                || !check_quality_value(record_index, qual)
            {
                println!("QC Result: FAIL");
                return;
            }

            line_buffer.clear();
        }
    }
    // println!("{}", line_num);
    if !line_buffer.is_empty() {
        println!("QC Result: FAIL");
        eprintln!(
            "FASTQ format error: incomplete record at end of file ({} extra lines)",
            line_buffer.len()
        );
        return;
    }

    println!("QC Result: PASS");

    if !non_atgc_counts.is_empty() {
        println!("\nNon-ATGC base report:");
        for (base, count) in non_atgc_counts.iter() {
            println!("  {}: {}", base, count);
        }
    } else {
        println!("\nAll bases are A, T, G, or C.");
    }
}

fn check_header(record_index: usize, header: &str) -> bool {
    if !(header.starts_with('@') || header.starts_with('>')) {
        eprintln!("Invalid header line (record {}): {}", record_index + 1, header);
        false
    } else {
        true
    }
}

fn check_duplicate_header(
    record_index: usize,
    header: &str,
    headers_seen: &mut HashSet<String>,
) -> bool {
    if !headers_seen.insert(header.to_string()) {
        eprintln!("Duplicate header found (record {}): {}", record_index + 1, header);
        false
    } else {
        true
    }
}

fn check_sequence_length(record_index: usize, seq: &str, qual: &str) -> bool {
    if seq.len() != qual.len() {
        eprintln!(
            "Length mismatch (record {}): seq = {}, qual = {}",
            record_index + 1,
            seq.len(),
            qual.len()
        );
        false
    } else {
        true
    }
}

fn check_non_atgc(seq: &str, non_atgc_counts: &mut HashMap<char, usize>) -> bool {
    for base in seq.chars() {
        let upper = base.to_ascii_uppercase();
        if !"ATGC".contains(upper) {
            *non_atgc_counts.entry(upper).or_insert(0) += 1;
        }
    }
    true
}

fn check_quality_value(record_index: usize, qual: &str) -> bool {
    let mut valid = true;
    for (j, c) in qual.chars().enumerate() {
        let ascii_value = c as u8;
        if ascii_value < 33 || ascii_value > 126 {
            eprintln!(
                "Invalid character in quality string (record {}, position {}): '{}'",
                record_index + 1,
                j + 1,
                c
            );
            valid = false;
        }
    }
    valid
}

