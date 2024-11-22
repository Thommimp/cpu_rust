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
        let program = fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;
        if self.data.len() < program.len() {
            return Err(String::from("Program is to large for the memory capacity"));
        }
        self.data[0..program.len()].copy_from_slice(&program);

        Ok(program.len())
    }

    pub fn print(&self) {
        const LINE_LEN: usize = 16;
        let mut repeating = false;
        let mut prev_line: Option<&[u8]> = None;
        for (i, line) in self.data.chunks(LINE_LEN).enumerate() {
            if let Some(prev) = prev_line {
                if prev == line {
                    if !repeating {
                        println!("*");
                        repeating = true;
                    }
                    continue;
                }
                if repeating {
                    print_line(i-1, prev);
                    repeating = false;
                }
            }

            print_line(i, line);
            prev_line = Some(line);
        }
    }

}

fn print_line(line_number: usize, line: &[u8]) {

    // Print address
    print!("{:08x}: ", line_number * 16);

    // Print hex bytes
    for byte in line.chunks_exact(2) {
        print!("{:02x}{:02x} ", byte[0], byte[1]);
    }

    for byte in line {
        if byte.is_ascii_graphic() || byte.is_ascii_whitespace() {
            print!("{}", *byte as char);
        } else {
            print!(".");
        }
    }

    print!("\n");
}
