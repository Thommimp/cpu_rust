use std::fs;

pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new(capacity: usize) -> Self {
        Memory {
            data: vec![0; capacity],
        }
    }

    fn read_byte(&self, addr: usize) -> u8 {
        return self.data[addr] as u8
    }

    fn read_halfword(&self, addr: usize) -> u16 {
        return u16::from_le_bytes(self.data[addr..addr + 2].try_into().unwrap());
    }

    fn read_word(&self, addr: usize) -> u32 {
        return u32::from_le_bytes(self.data[addr..addr + 4].try_into().unwrap());
    }

    fn write_byte(&mut self, addr: usize, data: u8) {
        self.data[addr] = data;
    }

    fn write_halfword(&mut self, addr: usize, data: u16) {
        self.data[addr..addr + 2].copy_from_slice(&data.to_le_bytes());
    }

    fn write_word(&mut self, addr: usize, data: u32) {
        self.data[addr..addr + 4].copy_from_slice(&data.to_le_bytes());
    }

    pub fn load_byte(&self, addr: u32) -> u32 {
        (self.read_byte(addr as usize) as i32) as u32
    }
    pub fn load_halfword(&self, addr: u32) -> u32 {
        (self.read_halfword(addr as usize) as i32) as u32
    }
    pub fn load_word(&self, addr: u32) -> u32 {
        self.read_word(addr as usize)
    }
    pub fn load_byte_unsigned(&self, addr: u32) -> u32 {
        self.read_byte(addr as usize) as u32
    }
    pub fn load_halfword_unsigned(&self, addr: u32) -> u32 {
        self.read_halfword(addr as usize) as u32
    }
    pub fn store_byte(&mut self, addr: u32, data: u32) {
        self.write_byte(addr as usize, data as u8)
    }
    pub fn store_halfword(&mut self, addr: u32, data: u32) {
        self.write_halfword(addr as usize, data as u16)
    }
    pub fn store_word(&mut self, addr: u32, data: u32) {
        self.write_word(addr as usize, data)
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
        println!("|======|==========|");
        println!("| addr | data     |");
        println!("|======|==========|");
        for (i, &next_word) in self.data.iter().enumerate() {
            if i != 0 {
                if word != next_word {
                    repeat = false;
                }
                if (prev_word == word) & (word == next_word) & !repeat {
                    println!("| {:>4} | {:^8} |", "...", "...");
                    repeat = true;
                } else if !repeat {
                    println!("| {:04x} | {:08x} |", (i - 1) * 4, word);
                }
            }
            prev_word = word;
            word = next_word;
        }
        println!(
            "| {:04x} | {:08x} |",
            (self.data.len() - 1) * 4,
            self.data.last().unwrap()
        );
        println!("|=======|==========|");
    }
}
