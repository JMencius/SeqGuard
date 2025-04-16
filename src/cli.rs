use clap::Parser;

/// Command-line arguments for seqguard
#[derive(Parser, Debug)]
#[command(name = "seqguard", version, about = "FASTQ quality check, based on Rust")]
pub struct Args {
    /// Path to .fastq or .fastq.gz file
    #[arg(short, long)]
    pub input: String,

}

