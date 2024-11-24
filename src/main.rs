mod memory;
mod utils;

use clap::{Parser, ValueEnum};
use elf::{endian::AnyEndian, ElfBytes};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// input file
    elf_file: String,

    /// verbose output
    #[arg(short)]
    verbose: bool,

    /// single step
    #[arg(short)]
    is_single_stop: bool,

    /// dump memory and register trace to dump.txt
    #[arg(short)]
    dump_history: bool,

    /// branch perdiction strategy
    #[arg(short = 'b', value_name = "param")]
    strategy: Option<BranchPredictorStrategy>,
}

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "UPPER")]
enum BranchPredictorStrategy {
    AT,
    NT,
    BTFNT,
    BPB,
}

fn main() {
    let args = Args::parse();
    let path = std::path::PathBuf::from(&args.elf_file);
    let file_data = std::fs::read(path).expect(&format!("Fail to open file {}", args.elf_file));
    let file = ElfBytes::<AnyEndian>::minimal_parse(&file_data)
        .expect(&format!("Fail to load ELF file {}", args.elf_file));

    let mut memory = memory::MemoryManager::new();
    utils::load_elf_to_memory(&file, &mut memory);

    if args.verbose {
        utils::print_elf_info(&file);
        memory.print_info();
    }
}
