mod args;
mod memory;
mod regs;
mod simulator;
mod utils;

use clap::Parser;
use elf::{endian::AnyEndian, ElfBytes};

fn main() {
    let args = args::Args::parse();
    let path = std::path::PathBuf::from(&args.elf_file);
    let file_data = std::fs::read(path).expect(&format!("Fail to open file {}", args.elf_file));
    let file = ElfBytes::<AnyEndian>::minimal_parse(&file_data)
        .expect(&format!("Fail to load ELF file {}", args.elf_file));

    let mut memory = memory::MemoryManager::new();
    utils::load_elf_to_memory(&file, &mut memory);

    if args.verbose {
        utils::print_elf_info(&file);
        // memory.print_info();
    }

    let sim = simulator::Simulator::new(&args, &mut memory, utils::get_entry(&file.ehdr));
}
