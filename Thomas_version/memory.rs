use std::fs;

pub struct Memory {
    pub data: Vec<u8>
}

impl Memory {
    pub fn new(capacity: usize) -> Self {
        Memory {
            data: vec![0; capacity / 4]
        }
    }

    pub fn read_byte(&self, address: u32) -> u8 {
        let index = address as usize;
        self.data[index]
    }

    pub fn read_halfword(&self, address: u32) -> u16 {
        let index = address as usize;
        (self.data[index] as u16)
            | ((self.data[index+1] as u16) << 8)
    }

    pub fn read_signed_byte(&self, address: u32) -> i8 {
        let index = address as usize;
        self.data[index] as i8 // Convert the unsigned byte to signed
    }

    // Function to read a signed halfword (i16)
    pub fn read_signed_halfword(&self, address: u32) -> i16 {
        let index = address as usize;
        let lower_byte = self.data[index] as u16; // Read the lower byte
        let upper_byte = self.data[index + 1] as u16; // Read the upper byte

        // Combine the bytes and convert to signed
        (lower_byte | (upper_byte << 8)) as i16 // Treat it as little-endian
    }


    pub fn read_word(&self, address: u32) -> u32 {
        let index = address as usize;
        (self.data[index] as u32)
            | ((self.data[index+1] as u32) << 8)
            | ((self.data[index+2] as u32) << 16)
            | ((self.data[index+3] as u32) << 24)
    }

        pub fn read_signed_word(&self, address: u32) -> i32 {
        let index = address as usize;
        let word = (self.data[index] as u32)
            | ((self.data[index + 1] as u32) << 8)
            | ((self.data[index + 2] as u32) << 16)
            | ((self.data[index + 3] as u32) << 24);
        
        word as i32 // Convert the final result to i32 (handles the sign bit)
    }

    pub fn write_byte(&mut self, address: u32, data: u8) {
        let index = address as usize;
        self.data[index] = data as u8;
    }

    pub fn write_signed_byte(&mut self, address: u32, data: i8) {
        let index = address as usize;
        self.data[index] = data as u8; // Cast i8 to u8 to store it in memory
    }

    pub fn write_halfword(&mut self, address: u32, data: u16) {
        let index = address as usize;
        self.data[index] = data as u8;
        self.data[index+1] = (data >> 8) as u8;
    }

      pub fn write_signed_halfword(&mut self, address: u32, data: i16) {
        let index = address as usize;
        self.data[index] = data as u8;
        self.data[index+1] = (data >> 8) as u8;
    }

    pub fn write_word(&mut self, address: u32, data: u32) {
        let index = address as usize;
        self.data[index] = data as u8;
        self.data[index+1] = (data >> 8) as u8;
        self.data[index+2] = (data >> 8) as u8;
        self.data[index+3] = (data >> 8) as u8;
    }

    pub fn write_signed_word(&mut self, address: u32, data: i32) {
        let index = address as usize;
        self.data[index] = data as u8;
        self.data[index+1] = (data >> 8) as u8;
        self.data[index+2] = (data >> 8) as u8;
        self.data[index+3] = (data >> 8) as u8;
    }
    
    pub fn load_from_file(&mut self, path: &str) -> Result<usize, String>{
        let program = fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;
        if self.data.len() < program.len() {
            return Err(String::from("Program is to large for the memory capacity"));
        }
        self.data[0..program.len()].copy_from_slice(&program);
            println!("First few bytes of loaded program: {:?}", &self.data[0..std::cmp::min(16, program.len())]);

        Ok(program.len()/4)
    }
}
