use std::io;

mod cpu;
mod isa;
mod memory;
mod register_file;

use cpu::Cpu;

const MEMORY_CAPACITY: usize = 4*1024;

fn main() -> io::Result<()> {
    // cunstruct the Cpu struct
    let mut cpu = Cpu::new(MEMORY_CAPACITY);

    // load binary file and handle error
    match cpu.load_file("ex2.bin") {
        Ok(i) => println!("{} bytes were loaded into memory", i),
        Err(e) => eprintln!("Error: {}", e),
    }

    // print the content of the memory
    cpu.print_memory();

    // tick loop - loops until Cpu.halt is true
    while !cpu.halt() {
        // executes one tick
        match cpu.tick() {
            Ok(()) => continue,
            Err(e) => eprintln!("{}", e),
        }
    }

    // print the content of registers and memory
    cpu.print_status();
    cpu.print_memory();

    Ok(())
}
