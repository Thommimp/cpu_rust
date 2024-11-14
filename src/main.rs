use std::io;

mod cpu;
mod memory;
mod register_file;
mod isa;

use cpu::Cpu;

const MEMORY_CAPACITY: usize = 4096;

fn main() -> io::Result<()> {

    let mut cpu = Cpu::new(MEMORY_CAPACITY);
    cpu.init();

    match cpu.load_file("ex2.bin") {
        Ok(i) => println!("{} bytes were loaded into memory", i),
        Err(e) => eprintln!("Error: {}", e),
    }

    cpu.print_memory();

    while !cpu.halt() {
        match cpu.tick() {
            Ok(()) => continue,
            Err(e) => eprintln!("{}",e),
        }
    }

    cpu.print_status();

    Ok(())
}
