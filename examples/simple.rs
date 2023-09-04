use std::fs;

use clap::Parser;
use riscv_elf_parse::Memory;
use xmas_elf::ElfFile;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to source file
    #[arg(short, long)]
    source_path: String,

    #[arg(short, long)]
    link_path: String,
}

fn main() {
    let args = Args::parse();
    let memory = if cfg!(target_os = "windows") {
        Memory::new_from_assembly(&args.source_path, &args.link_path, "riscv-none-")
    } else {
        Memory::new_from_assembly(&args.source_path, &args.link_path, "riscv32-unknown-elf-")
    };
    println!("{:?}", memory.symbols);
    println!("{}", memory);
}
