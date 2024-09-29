
// use std::fs::File;
// use std::io::{self, BufRead};
// use std::path::Path;

// fn read_binary(file_path: &str, output: &mut [u32; 100]) -> io::Result<usize> {
//     // Specify the path to the text file
//     let path = Path::new(file_path);

//     // Open the file
//     let file = File::open(&path)?;

//     // Create a buffered reader
//     let reader = io::BufReader::new(file);
    
//     // Initialize a counter for the number of instructions read
//     let mut count = 0;

//     // Read lines from the file
//     for line in reader.lines() {
//         if count >= output.len() {
//             return Err(io::Error::new(io::ErrorKind::Other, "Memory array is full."));
//         }

//         // Convert the binary string to an unsigned 32-bit integer
//         output[count] = u32::from_str_radix(&line?.trim(), 2)
//             .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

//         count += 1; // Increment the count
//     }

//     Ok(count) // Return the number of instructions read
// }

// fn main() -> io::Result<()> {
//     // Variable to hold up to 100 unsigned integers
//     let mut memory: [u32; 100] = [0; 100];

//     // Call the read_binary function
//     match read_binary("binary.txt", &mut memory) {
//         Ok(count) => {
//             println!("Read {} instructions from the binary file:", count);
//             for i in 0..count {
//                 println!("Memory[{}]: {}", i, memory[i]);
//             }
//         },
//         Err(e) => {
//             eprintln!("Error: {}", e);
//         }
//     }

//     Ok(())
// }

use std::fs::File;
use std::io::{self, Read, BufReader};

fn read_binary(file_path: &str, output: &mut [u32; 100]) -> io::Result<usize> {
    // Specify the path to the text file
    let path = std::path::Path::new(file_path);

    // Open the file
    let file = File::open(&path)?;
    
    // Create a buffered reader
    let mut reader = BufReader::new(file);

    // Read the entire file into a string
    let mut binary_string = String::new();
    reader.read_to_string(&mut binary_string)?;

    // Remove any whitespace (if there happens to be any)
    let binary_string = binary_string.trim();

    // Check if the length is a multiple of 32
    let length = binary_string.len();
    println!("{:?}", length);
    if length % 32 != 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Binary data is not a multiple of 32 bits."));
    }

    // Initialize a counter for the number of instructions read
    let mut count = 0;

    // Read 32 bits at a time
    for chunk in binary_string.as_bytes().chunks(32) {
        if count >= output.len() {
            return Err(io::Error::new(io::ErrorKind::Other, "Memory array is full."));
        }

        // Convert the 32-bit binary chunk to a string
        let binary_str = std::str::from_utf8(chunk).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, e)
        })?;

        // Convert the binary string to an unsigned 32-bit integer
        output[count] = u32::from_str_radix(binary_str, 2)
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







