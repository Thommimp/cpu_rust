use std::io;

mod cpu;
mod isa;
mod memory;
mod register_file;

use cpu::Cpu;

const MEMORY_CAPACITY: usize = 1000 * 1024;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let file_path = &args[1];

    println!("File path: {}", file_path);

    // cunstruct the Cpu struct
    let mut cpu = Cpu::new(MEMORY_CAPACITY);

    // load binary file and handle error
    match cpu.load_file(file_path) {
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
