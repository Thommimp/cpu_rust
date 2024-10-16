use crate::isa::Instruction;

pub struct Pipeline {
    ifid: IfIdRegister,
    idex: IdExRegister,
    exmem: ExMemRegiser,
    memwb: MemWBRegister,
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline {
            ifid: IfIdRegister::new(),
            idex: IdExRegister::new(),
            exmem: ExMemRegiser::new(),
            memwb: MemWBRegister::new(),
        }
    }
    pub fn tick(&mut self) {
        self.memwb.inst = self.exmem.inst.clone();
        self.memwb.alu_result = self.exmem.alu_result;

        self.exmem.inst = self.idex.inst.clone();

        self.idex.pc = self.ifid.pc;
    }
    pub fn update_ifid(&mut self, pc: u32, inst_word: u32) {
        self.ifid.pc = pc;
        self.ifid.inst_word = inst_word;
    }
    pub fn get_inst_word(&self) -> u32 {
        self.ifid.inst_word
    }
    pub fn update_idex( &mut self,
        inst: Option<Instruction>,
        reg1: u32,
        reg2: u32,
        imm: u32,
        rd: usize,
    ) {
        self.idex.inst = inst;
        self.idex.reg1 = reg1;
        self.idex.reg2 = reg2;
        self.idex.imm = imm;
        self.idex.rd = rd;
    }
    pub fn get_execute_instruction(&self) -> Option<Instruction> {
        self.idex.inst.clone()
    }
    pub fn get_execute_arguments(&self) -> (u32, u32, u32) {
        (self.idex.reg1, self.idex.reg2, self.idex.imm)
    }
    pub fn update_exmem(&mut self, alu_result: u32) {
        self.exmem.alu_result = alu_result;
    }
    pub fn update_branch_pc(&mut self, pc_sum: u32) {
        self.exmem.pc_sum = pc_sum;
    }
    pub fn set_branch(&mut self) {
        self.exmem.branch = true;
    }
    pub fn get_memory_instruction(&self) -> Option<Instruction> {
        self.exmem.inst.clone()
    }
    pub fn get_memory_arg(&self) -> (u32, u32) {
        (self.exmem.alu_result, self.exmem.write_data)
    }
    pub fn update_memwb(&mut self, memory: u32) {
        self.memwb.memory = memory;
    }
    pub fn get_writeback_instruction(&self) -> Option<Instruction> {
        self.memwb.inst.clone()
    }
    pub fn get_writeback_arg(&self) -> (u32, u32, usize) {
        (
            self.memwb.alu_result,
            self.memwb.memory,
            self.memwb.rd,
        )
    }
    pub fn print_status(&self) {
        let idex_inst = match self.idex.inst.clone() {
            Some(i) => i.name,
            None => "None",
        };
        let exmem_inst = match self.exmem.inst.clone() {
            Some(i) => i.name,
            None => "None",
        };
        let memwb_inst = match self.memwb.inst.clone() {
            Some(i) => i.name,
            None => "None",
        };
        println!("             |  IF/ID   |  ID/EX   |  EX/MEM  |  MEM/WB  | ");
        println!("             |==========|==========|==========|==========| ");
        println!("          pc | {:08X  } | {:08X  } | {:08X  } |        - | ", self.ifid.pc, self.idex.pc, self.exmem.pc_sum);
        println!("      branch |        - |        - | {:8    } |        - | ", self.exmem.branch);
        println!("      i_word | {:08X  } |        - |        - |        - | ", self.ifid.inst_word);
        println!("        inst |        - | {:^8   } | {:^8   } | {:^8   } | ", idex_inst, exmem_inst, memwb_inst);
        println!("          rd |        - | {:08X  } | {:08X  } | {:08X  } | ", self.idex.rd, self.exmem.rd, self.memwb.rd);
        println!("        reg1 |        - | {:08X  } |        - |        - | ", self.idex.reg1);
        println!("        reg2 |        - | {:08X  } |        - |        - | ", self.idex.reg2);
        println!("         imm |        - | {:08X  } |        - |        - | ", self.idex.imm);
        println!("  alu_result |        - |        - | {:08X  } | {:08X  } | ", self.exmem.alu_result, self.memwb.alu_result);
        println!("  write_data |        - |        - | {:08X  } |        - | ", self.exmem.write_data);
        println!("      memory |        - |        - | {:08X  } |        - | ", self.memwb.memory);
    }
}

struct IfIdRegister {
    pc: u32,
    inst_word: u32,
}

impl IfIdRegister {
    pub fn new() -> Self {
        IfIdRegister {
            pc: 0,
            inst_word: 0,
        }
    }
}

struct IdExRegister {
    pc: u32,
    inst: Option<Instruction>,
    reg1: u32,
    reg2: u32,
    imm: u32,
    rd: usize,
}

impl IdExRegister {
    pub fn new() -> Self {
        IdExRegister {
            pc: 0,
            inst: None,
            reg1: 0,
            reg2: 0,
            imm: 0,
            rd: 0,
        }
    }
}

struct ExMemRegiser {
    pc_sum: u32,
    branch: bool,
    inst: Option<Instruction>,
    alu_result: u32,
    write_data: u32,
    rd: usize,
}

impl ExMemRegiser {
    pub fn new() -> Self {
        ExMemRegiser {
            pc_sum: 0,
            branch: false,
            inst: None,
            alu_result: 0,
            write_data: 0,
            rd: 0,
        }
    }
}

struct MemWBRegister {
    inst: Option<Instruction>,
    alu_result: u32,
    memory: u32,
    rd: usize,
}
    
impl MemWBRegister {
    pub fn new() -> Self {
        MemWBRegister {
            inst: None,
            alu_result: 0,
            memory: 0,
            rd: 0,
        }
    }
}


