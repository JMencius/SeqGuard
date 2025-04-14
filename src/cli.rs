use clap::Parser;

/// Command-line arguments for seqguard
#[derive(Parser, Debug)]
#[command(name = "seqguard", version, about = "FASTQ quality check, based on Rust")]
pub struct Args {
    /// Path to .fastq or .fastq.gz file
    #[arg(short, long)]
    pub input: String,

    /// Number of threads to use
    #[arg(short, long, default_value_t = 8)]
    pub threads: usize,
}

