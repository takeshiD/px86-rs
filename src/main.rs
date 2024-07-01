mod instruction;
mod emulator;
use std::path::PathBuf;
use clap::Parser;
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file path to analyze file maybe included MBR/PBR.
    #[arg(value_name="FILE")]
    infile: PathBuf,
}

fn main() {
    let args = Args::parse();
    let f = args.infile;
}
