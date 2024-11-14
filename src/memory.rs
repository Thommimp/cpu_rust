use std::fs;

pub struct Memory {
    data: Vec<u32>,
}

impl Memory {
    pub fn new(capacity: usize) -> Self {
        Memory {
            data: vec![0; capacity / 4],
        }
    }

    fn read_byte(&self, address: u32) -> u8 {
        let index = (address / 4) as usize;
        let offset = 8 * (3 - (address % 4));
        (self.data[index] >> offset) as u8
    }

    fn read_halfword(&self, address: u32) -> u16 {
        let index = (address / 4) as usize;
        let offset = 16 * (3 - (address % 2));
        (self.data[index] >> offset) as u16
    }

    fn read_word(&self, address: u32) -> u32 {
        let index = (address / 4) as usize;
        self.data[index] as u32
    }

    fn write_byte(&mut self, address: u32, data: u8) {
        let index = (address / 4) as usize;
        let offset = 8 * (address % 4);
        self.data[index] |= (data as u32) << offset;
    }

    fn write_halfword(&mut self, address: u32, data: u16) {
        let index = (address / 4) as usize;
        let offset = 16 * (address % 2);
        self.data[index] |= (data as u32) << offset;
    }

    fn write_word(&mut self, address: u32, data: u32) {
        let index = (address / 4) as usize;
        self.data[index] = data;
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

    pub fn load_file(&mut self, path: &str) -> Result<usize, String> {
        let file = fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;
        if self.data.len() < file.len() {
            return Err(String::from("Program is to large for the memory capacity"));
        }
        assert!(file.len() % 4 == 0, "Length of file is not correct");
        let data: Vec<u32> = file
            .chunks_exact(4)
            .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
            .collect();

        self.data[0..data.len()].copy_from_slice(&data);

        Ok(file.len())
    }

    pub fn print(&self) {
        let mut prev_word: u32 = 0;
        let mut word: u32 = 0;
        let mut repeat: bool = false;
        println!("|=======|==========|");
        println!("| addr  | data     |");
        println!("|=======|==========|");
        for (i, &next_word) in self.data.iter().enumerate() {
            if i != 0 {
                if word != next_word {
                    repeat = false;
                }
                if (prev_word == word) & (word == next_word) & !repeat {
                    println!("| {:>5} | {:^8} |", "...", "...");
                    repeat = true;
                } else if !repeat {
                    println!("| {:5} | {:08x} |", (i - 1)*4, word);
                }
            }
            prev_word = word;
            word = next_word;
        }
        println!(
            "| {:5} | {:08x} |",
            (self.data.len()-1)*4,
            self.data.last().unwrap()
        );
        println!("|=======|==========|");
    }
}
