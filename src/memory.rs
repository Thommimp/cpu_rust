use std::fs;

pub struct Memory {
    data: Vec<u8>
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

    pub fn read_word(&self, address: u32) -> u32 {
        let index = address as usize;
        (self.data[index] as u32)
            | ((self.data[index+1] as u32) << 8)
            | ((self.data[index+2] as u32) << 16)
            | ((self.data[index+3] as u32) << 24)
    }

    pub fn write_byte(&mut self, address: u32, data: u8) {
        let index = address as usize;
        self.data[index] = data as u8;
    }

    pub fn write_halfword(&mut self, address: u32, data: u16) {
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
    
    pub fn load_byte(&self, address: u32) -> u32 {
        (self.read_byte(address) as i32) as u32
    }
    pub fn load_halfword(&self, address: u32) -> u32 {
        (self.read_halfword(address) as i32) as u32
    }
    pub fn load_word(&self, address: u32) -> u32 {
        self.read_word(address)
    }
    pub fn load_byte_unsigned(&self, address: u32) -> u32 {
        self.read_byte(address) as u32
    }
    pub fn load_halfword_unsigned(&self, address: u32) -> u32 {
        self.read_halfword(address) as u32
    }
    pub fn store_byte(&mut self, address: u32, data: u32) {
        self.write_byte(address, data as u8)
    }
    pub fn store_halfword(&mut self, address: u32, data: u32) {
        self.write_halfword(address, data as u16)
    }
    pub fn store_word(&mut self, address: u32, data: u32) {
        self.write_word(address, data)
    }
    
    pub fn load_from_file(&mut self, path: &str) -> Result<usize, String>{
        let program = fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;
        if self.data.len() < program.len() {
            return Err(String::from("Program is to large for the memory capacity"));
        }
        self.data[0..program.len()].copy_from_slice(&program);
        Ok(program.len()/4)
    }
}
