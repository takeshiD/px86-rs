mod instruction;
mod emulator;
use emulator::Emulator;
use std::path::PathBuf;
use clap::Parser;
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file path to analyze file maybe included MBR/PBR.
    #[arg(value_name="FILE")]
    infile: PathBuf,
    #[arg(short, long, default_value_t = 1024*1024, value_name="RAM")]
    ram_size: usize,
}

fn main() {
    let args = Args::parse();
    // let infile = args.infile;
    let ram_size = args.ram_size;
    let emu = Emulator::new(ram_size, 0x7c00, 0x7c00);
    println!("{}", emu);
}
