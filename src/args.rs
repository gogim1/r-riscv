
use clap::{Parser, ValueEnum};


#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// input file
    pub elf_file: String,

    /// verbose output
    #[arg(short)]
    pub verbose: bool,

    /// single step
    #[arg(short)]
    pub is_single_stop: bool,

    /// dump memory and register trace to dump.txt
    #[arg(short)]
    pub dump_history: bool,

    /// branch perdiction strategy
    #[arg(short = 'b', value_name = "param")]
    pub strategy: Option<BranchPredictorStrategy>,
}

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "UPPER")]
pub enum BranchPredictorStrategy {
    AT,
    NT,
    BTFNT,
    BPB,
}