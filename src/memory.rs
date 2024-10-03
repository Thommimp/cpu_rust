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
    
    pub fn load_from_file(&mut self, path: &str){
        let program = fs::read(path).expect("Unable to read binary file");
        assert!(self.data.len() >= program.len(), "Program is to large for the memory capacity");
        self.data[0..program.len()].copy_from_slice(&program)
    }
}
