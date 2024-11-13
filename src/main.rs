use std::io;

mod cpu;
mod memory;
mod isa;
mod pipeline;

use cpu::Cpu;

const MEMORY_CAPACITY: usize = 4096;

fn main() -> io::Result<()> {

    let mut cpu = Cpu::new(MEMORY_CAPACITY);


    match cpu.load_file("example.bin") {
        Ok(i) => {
            println!("{} bytes were loaded into memory", i)
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
