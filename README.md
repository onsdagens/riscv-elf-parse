# RISC-V ELF Parser

WIP !

The example ``simple.rs`` will compile the RISC-V assembly source file passed under the
`` --source-path `` flag, link it according to script passed under ``--link-path`` , and generate
a ``Memory`` struct

It may be used on example assembly and linker script using

``` cargo run --example simple -- --source-path test.s --link-path memory.x ```
