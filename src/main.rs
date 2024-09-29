
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_binary(file_path: &str, output: &mut [u32; 100]) -> io::Result<usize> {
    // Specify the path to the text file
    let path = Path::new(file_path);

    // Open the file
    let file = File::open(&path)?;

    // Create a buffered reader
    let reader = io::BufReader::new(file);
    
    // Initialize a counter for the number of instructions read
    let mut count = 0;

    // Read lines from the file
    for line in reader.lines() {
        if count >= output.len() {
            return Err(io::Error::new(io::ErrorKind::Other, "Memory array is full."));
        }

        // Convert the binary string to an unsigned 32-bit integer
        output[count] = u32::from_str_radix(&line?.trim(), 2)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        count += 1; // Increment the count
    }

    Ok(count) // Return the number of instructions read
}

fn main() -> io::Result<()> {
    // Variable to hold up to 100 unsigned integers
    let mut memory: [u32; 100] = [0; 100];

    // Call the read_binary function
    match read_binary("binary.txt", &mut memory) {
        Ok(count) => {
            println!("Read {} instructions from the binary file:", count);
            for i in 0..count {
                println!("Memory[{}]: {}", i, memory[i]);
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}






