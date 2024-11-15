use crate::isa::get_register_alias;

pub struct RegisterFile {
    x: [u32; 32],
}

impl RegisterFile {
    pub fn new() -> Self {
        RegisterFile { x: [0; 32] }
    }

    pub fn set(&mut self, index: usize, value: u32) {
        if index < 32 {
            self.x[index] = value;
        }
    }

    pub fn get(&self, index: usize) -> u32 {
        if index < 32 {
            self.x[index]
        } else {
            0
        }
    }
    pub fn print(&self) {
        println!("|------------------|");
        for (i, &value) in self.x.iter().enumerate().rev() {
            println!("| {:>4}:  0x{:08x}|", get_register_alias(i), value);
        }
    }
}
