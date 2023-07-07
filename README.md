# ELF file disassembler

WIP !

the example file ``test.s`` may be compiled via ``` riscv32-unknown-elf-gcc -ggdb3 -c test.s -o output.o ```

It may then be disassembled with this utility by 

``` cargo run -- --path ./output.o ```