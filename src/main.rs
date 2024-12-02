use std::fs;
use std::io;

mod cpu;
mod isa;
mod memory;
mod register_file;

use cpu::Cpu;

const MEMORY_CAPACITY: usize = 1200 * 1024;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let bin_file_path = &args[1];
    let res_file_path;
    let res_test;

    println!("Bin file path: {}", bin_file_path);
    if args.len() == 3 {
        res_test = true;
    } else {
        res_test = false;
    };

    println!("Arg {}", args.len());

    // cunstruct the Cpu struct
    let mut cpu = Cpu::new(MEMORY_CAPACITY);

    // load binary file and handle error
    match cpu.load_file(bin_file_path) {
        Ok(i) => println!("{} bytes were loaded into memory", i),
        Err(e) => eprintln!("Error: {}", e),
    }

    match cpu.load_file(bin_file_path) {
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

    if res_test {
        let reg_dump = cpu.dump_registers();
        res_file_path = &args[2];
        let res = read_res_bin(res_file_path);

        if reg_dump == res.unwrap() {
            println!("OK – all register values match the res binary")
        } else {
            println!("ERR – some register values do not match the res binary")
        }
    }

    Ok(())
}

fn read_res_bin(path: &str) -> Result<[u32; 32], String> {
    let file = fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;

    if file.len() != 32 * 4 {
        return Err(String::from("File is not exacly 128 bytes"));
    }

    let mut res: [u32; 32] = [0; 32];

    for (i, chunk) in file.chunks_exact(4).enumerate() {
        res[i] = u32::from_le_bytes(chunk.try_into().unwrap());
    }

    Ok(res)
}
