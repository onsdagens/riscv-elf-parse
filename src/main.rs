use clap::Parser;
use std::fs;
use riscv32i_isa::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Path to file
   #[arg(short, long)]
   path: String,
}
struct ELF32 {
    bytes:Vec<u8>,
    instructions:Vec<Instruction>,
}
struct ELF64{
    bytes:Vec<u8>,
    
}
trait ELF{
    fn check_magic(&self);
    fn print_instructions(&self);
    fn push_instructions(&mut self); 
    fn is64bit(&self)->bool;
    fn is_le(&self)->bool;
    fn is_riscv(&self)->bool;
    fn new(bytes:Vec<u8>)->Self;
    fn entry_point(&self)->u32;
    fn program_header_offset(&self)->u32;
    fn section_header_offset(&self)->u32;
    fn program_header_entry_size(&self)->u16;
    fn program_header_entry_amt(&self)->u16;
    fn section_header_entry_size(&self)->u16;
    fn section_header_entry_amt(&self)->u16;
    fn executable_sections(&self)->Vec<(u32,u32)>;
}
impl ELF for ELF32{
    fn new(bytes:Vec<u8>)->ELF32{
        let mut elf  = ELF32{bytes:bytes,instructions:vec![]};
        elf.check_magic();
        elf.push_instructions();
        elf
    }
    fn print_instructions(&self){
        println!("Instruction opcodes in file order:");
        for instruction in &self.instructions[..]{
            println!("{:?}", instruction)
        }
    }
    fn push_instructions(&mut self){
        let sections = self.executable_sections();
        for section in sections{
            let size = section.1;
            let offset = section.0;
            let mut i = 0;
            while i<size {
                let segment = &self.bytes[(offset+i) as usize..(offset+i+4) as usize];
                let instruction_bits =
                match self.is_le(){
                    true=>u32::from_le_bytes(segment.try_into().unwrap()),
                    false=>u32::from_be_bytes(segment.try_into().unwrap()),
                };
                let opcode = bits_to_opcode((instruction_bits&0b1111111) as u8);
                self.instructions.push(Instruction::new(opcode));
                i+=4;
            }
        }

    }
    fn check_magic(&self){
        let magic:u32 = (self.bytes[0] as u32) <<24 | (self.bytes[1] as u32) <<16 | (self.bytes[2] as u32) <<8 | (self.bytes[3] as u32);
        if magic!=0x7f454c46{
            panic!("The provided file does not contain a valid ELF header!");
        }
    }
    fn is64bit(&self)->bool{
        let flag = self.bytes[0x4];
        match flag{
            1=>false,
            2=>true,
            _=>panic!("The 64-bit flag in the header was invalid!")
        }
    }
    fn is_le(&self)->bool{
        let flag = self.bytes[0x5];
        match flag{
            1=>true,
            2=>false,
            _=>panic!("The endianness in the header was invalid!")
        }
    }
    fn is_riscv(&self)->bool{
        let flag = self.bytes[0x12];
        match flag{
            0xF3=>true,
            _=>false,
        }
    }
    fn entry_point(&self)->u32{
        let segment = &self.bytes[0x18..0x18+4];
        match self.is_le(){
            true=>u32::from_le_bytes(segment.try_into().unwrap()),
            false=>u32::from_be_bytes(segment.try_into().unwrap()),
        }
    }
    fn program_header_offset(&self)->u32{
        let segment = &self.bytes[0x1c..0x1c+4];
        match self.is_le(){
            true=>u32::from_le_bytes(segment.try_into().unwrap()),
            false=>u32::from_be_bytes(segment.try_into().unwrap()),
        }
    }
    fn section_header_offset(&self)->u32{
        let segment = &self.bytes[0x20..0x20+4];
        match self.is_le(){
            true=>u32::from_le_bytes(segment.try_into().unwrap()),
            false=>u32::from_be_bytes(segment.try_into().unwrap()),
        }
    }
    fn program_header_entry_size(&self)->u16{
        let segment = &self.bytes[0x2a..0x2a+2];
        match self.is_le(){
            true=>u16::from_le_bytes(segment.try_into().unwrap()),
            false=>u16::from_be_bytes(segment.try_into().unwrap()),
        }
    }
    fn program_header_entry_amt(&self)->u16{
        let segment = &self.bytes[0x2c..0x2c+2];
        match self.is_le(){
            true=>u16::from_le_bytes(segment.try_into().unwrap()),
            false=>u16::from_be_bytes(segment.try_into().unwrap()),
        }
    }
    fn section_header_entry_size(&self)->u16{
        let segment = &self.bytes[0x2e..0x2e+2];
        match self.is_le(){
            true=>u16::from_le_bytes(segment.try_into().unwrap()),
            false=>u16::from_be_bytes(segment.try_into().unwrap()),
        }
    }
    fn section_header_entry_amt(&self)->u16{
        let segment = &self.bytes[0x30..0x30+2];
        match self.is_le(){
            true=>u16::from_le_bytes(segment.try_into().unwrap()),
            false=>u16::from_be_bytes(segment.try_into().unwrap()),
        }
    }
    // fn instruction_location(&self)->u32{
    //     if self.program_header_offset()!=0{ //if a program header exists, the instructions follow that.
    //         self.program_header_offset() + (self.program_header_entry_amt() as u32 * self.program_header_entry_size() as u32)
    //     }
    //     else{ //else, instructions follow the ELF file header
    //         let segment = &self.bytes[0x28..0x28+2];
    //         match self.is_le(){
    //             true=>u32::from_le_bytes([segment[0],segment[1],0,0]),
    //             false=>u32::from_be_bytes([0,0,segment[1],segment[0]]),
    //         }

    //     }
    // }
    fn executable_sections(&self)->Vec<(u32,u32)>{
        let mut sections = vec![];
        for i in 0..(self.section_header_entry_amt() as usize){
            let current_entry = self.section_header_offset() as usize + i*self.section_header_entry_size() as usize;
            let flags =
            match self.is_le(){
                true=>u32::from_le_bytes(self.bytes[current_entry+0x8 .. current_entry+0x8+0x4].try_into().unwrap()),
                false=>u32::from_be_bytes(self.bytes[current_entry+0x8 .. current_entry+0x8+0x4].try_into().unwrap()),
            };
            if flags>>2 & 0x1 == 1 {
                println!("Section {} is executable", i);
                let offset_slice = &self.bytes[current_entry+0x10..current_entry+0x10+0x4];
                let size_slice = &self.bytes[current_entry+0x14..current_entry+0x14+0x4];
                match self.is_le(){
                    true=>sections.push((u32::from_le_bytes(offset_slice.try_into().unwrap()),u32::from_le_bytes(size_slice.try_into().unwrap()))),
                    false=>sections.push((u32::from_be_bytes(offset_slice.try_into().unwrap()),u32::from_be_bytes(size_slice.try_into().unwrap()))),
                }
            }
            else{
                println!("Section {} is not executable",i);
            }
        }
        sections
    }

}

fn main(){
    let args = Args::parse();
    println!("{}",args.path);
    let bytes = fs::read(&args.path).unwrap();
    let elf:ELF32 = ELF32::new(bytes);
    println!("is 64-bit: {}", elf.is64bit());
    println!("is LE: {}", elf.is_le());
    println!("is RISC-V: {}", elf.is_riscv());
    println!("entry point: 0x{:x}", elf.entry_point());
    println!("program header table offset: 0x{:x}", elf.program_header_offset());
    println!("program header table entry size: {} bytes", elf.program_header_entry_size());
    println!("program header table entry amount: {} entries", elf.program_header_entry_amt());
    println!("section header offset: 0x{:x}", elf.section_header_offset());
    println!("section header table entry size: {} bytes", elf.section_header_entry_size());
    println!("section header table entry amount: {} entries", elf.section_header_entry_amt());
    println!("Opcode of 0b1100011:{:?}",bits_to_opcode(0b1100011));
    println!("Executable sections: {:?}", elf.executable_sections());
    elf.print_instructions();
}