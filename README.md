# SeqGuard
SeqGuard is a Rust-based program for FASTQ checking, mainly for Phred+33 FASTQ. It checks for:
1. FASTQ 4-line format or empty file
2. Invalid nucleotide (non-ATGC character)
3. Invalid QV character (ASCII <0 or > 93)
4. Mismatch length of sequence and QV
5. Duplicate headers


## Installation
Download a ready-to-use binary from the release page.

You may have to change the file permissions with `chmod +x seqgurad;`


## Usage
Only `-i` or `--input` is needed, output to standard output
```
FASTQ quality check, based on Rust

Usage: seqguard [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>      Path to .fastq or .fastq.gz file
  -h, --help               Print help
  -V, --version            Print version
``` 


### Examples
```
seqgurad -i test.fastq;
seqgurad -i test.fastq > result.txt;
```


## Test data
Small FASTQ files are provided (here)[./tests]. You can use it as input for testing.


## Resource consumption
For a 40G FASTQ file, memory peak at ~2G, finished in ~1 minute.

The actual performance may vary depending on factors such as I/O speed, memory speed, and CPU capabilities.


## Other information
Developed on `rustc 1.77.2 (25ef9e3d8 2024-04-09)`, tested on single AMD EPYC 7K62, 256G of DDR4 2400 RAM, and SATA SSD storage.
