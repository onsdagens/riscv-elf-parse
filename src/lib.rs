use std::{collections::{BTreeMap, HashMap}, fmt::Display, fs, process::Command};

use xmas_elf::{
    program::{SegmentData, Type},
    ElfFile,
};

pub struct Memory {
    pub bytes: BTreeMap<usize, u8>,
    pub symbols: HashMap<usize, String>,
}
impl Memory {
    fn get_symbols(file_data: &Vec<u8>) -> HashMap<usize, String>{
        use elf::ElfBytes;
        use elf::endian::AnyEndian;
        let slice = file_data.as_slice();
        let file = ElfBytes::<AnyEndian>::minimal_parse(slice).expect("");
        let common = file.find_common_data().expect("");
        let strtab = common.symtab_strs.unwrap();
        let mut symbols:HashMap<usize, String> = HashMap::new();
        for sym in common.symtab.unwrap().iter(){
            if file.section_headers().unwrap().get(sym.st_shndx as usize).unwrap_or(file.section_headers().unwrap().get(0).unwrap()).sh_flags & 0x2 == 0x2{ //if corresponding section lives in target memory, if unwrap_or_else fails, the section is invalid, so pass default index 0 section
                    if strtab.get(sym.st_name as usize).unwrap()!=""{ //if label name isn't empty
                        symbols.insert( sym.st_value as usize,strtab.get(sym.st_name as usize).unwrap().to_string());
                    }
            }
        }
        symbols
    }

    pub fn new_from_assembly(source_path: &str, link_path: &str, toolchain_prefix: &str, le: bool) -> Memory {
        let mut binding = Command::new(format!("{}as", toolchain_prefix));
        let compile = binding
            .current_dir("./")
            .arg(source_path)
            .arg("-o")
            .arg("output.o");
        compile.status().unwrap();
        let mut binding = Command::new(format!("{}ld", toolchain_prefix));
        let link = binding
            .current_dir("./")
            .arg("-o")
            .arg("output_linked.o")
            .arg("-T")
            .arg(link_path)
            .arg("output.o");
        link.status().unwrap();
        let bytes = fs::read("output_linked.o").unwrap();
        //let elf = ElfFile::new(&bytes).unwrap();
        Memory::new_from_file(&bytes, le)
    }

    pub fn new_from_file(elf_file: &Vec<u8>, le: bool) -> Memory {
        let elf = ElfFile::new(elf_file).unwrap();
        let mut memory = Memory {
            bytes: BTreeMap::new(),
            symbols: Self::get_symbols(elf_file),
        };
        for segment in elf.program_iter() {
            if segment.get_type().unwrap() == Type::Load {
                let data = segment.get_data(&elf).unwrap();
                match data {
                    SegmentData::Undefined(arr) => {
                        let chunks = arr.chunks_exact(4);
                        for (i, word) in chunks.enumerate() {
                            if le {
                                let le_int = u32::from_le_bytes(word.try_into().unwrap());
                                let le_bytes = le_int.to_be_bytes();
                                for (j, byte) in le_bytes.iter().enumerate() {
                                    memory.bytes.insert(
                                        segment.virtual_addr() as usize + i * 4 + j,
                                        byte.clone(),
                                    );
                                }
                            }
                            else{
                                let le_int = u32::from_le_bytes(word.try_into().unwrap());
                                let le_bytes = le_int.to_le_bytes();
                                for(j,byte) in le_bytes.iter().enumerate(){
                                    memory.bytes.insert(
                                        segment.virtual_addr() as usize + i * 4 + j,
                                        byte.clone(),
                                    );
                                }
                                
                            }
                        }
                    }
                    _ => panic!(),
                }
            }
        }
        memory
    }
}
impl Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, byte) in self.bytes.iter().enumerate() {
            if i % 4 == 0 {
                if i != 0 {
                    writeln!(f, "").ok();
                }
                write!(f, "Address 0x{:08x}:     ", byte.0).ok();
                write!(f, "0x").ok();
            }
            write!(f, "{:02x}", byte.1).ok();
        }
        writeln!(f, "")
    }
}
