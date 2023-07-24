use clap::Parser;
use riscv_elf_parse::Memory;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Path to source file
   #[arg(short, long)]
   source_path: String,

   #[arg(short, long)]
   link_path: String,
}

fn main(){
    let args = Args::parse();
    let memory = Memory::new_from_assembly(&args.source_path, &args.link_path);
    println!("{}", memory);
}

