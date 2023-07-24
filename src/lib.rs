use std::{collections::BTreeMap, fmt::Display, process::Command, fs};

use xmas_elf::{ElfFile, program::{Type, SegmentData}};

pub struct Memory{
    bytes: BTreeMap<usize,u8>,
}
impl Memory{
    fn insert(&mut self,byte:&u8, addr:&usize){
        self.bytes.insert(addr.clone(), byte.clone());
    }

    pub fn new_from_assembly(source_path:&str, link_path:&str)->Memory{
        let mut binding = Command::new("riscv32-unknown-elf-gcc");
        let compile = binding.current_dir("./")
        .arg("-ggdb3")
        .arg("-c")
        .arg(source_path)
        .arg("-o")
        .arg("output.o");
        compile.status().unwrap();
        let mut binding = Command::new("riscv32-unknown-elf-ld");
        let link = binding.current_dir("./")
        .arg("-o")
        .arg("output_linked.o")
        .arg("-T")
        .arg(link_path)
        .arg("output.o");
        link.status().unwrap();
        let bytes = fs::read("output_linked.o").unwrap();
        let elf = ElfFile::new(&bytes).unwrap();
        
        let mut memory = Memory{bytes:BTreeMap::new()};
        for segment in elf.program_iter(){
                if segment.get_type().unwrap() == Type::Load {
                    let data = segment.get_data(&elf).unwrap();
                    match data{
                        SegmentData::Undefined(arr) =>{
                            for (i, byte) in arr.iter().enumerate(){
                                memory.insert(byte, &(segment.virtual_addr() as usize+(i)));
                            }
                        },
                        _=>panic!()
                    }
                }
        }
        memory
    }

}
impl Display for Memory{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i,byte) in self.bytes.iter().enumerate(){
            if i % 4 == 0{
                writeln!(f,"").ok();
                write!(f,"Address 0x{:08x}:     ", byte.0).ok();
                write!(f,"0x").ok();
            }
            write!(f,"{:02x}", byte.1).ok();
        }
        writeln!(f,"")
    }
}