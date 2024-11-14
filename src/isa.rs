const RD_POS: usize = 7;
const RS1_POS: usize = 15;
const RS2_POS: usize = 20;
const OP_MASK: u32 = 0x0000007F;
const RD_MASK: u32 = 0x00000F80;
const RS1_MASK: u32 = 0x000F8000;
const RS2_MASK: u32 = 0x01F00000;
const FN3_MASK: u32 = 0x00007000;
const FN7_MASK: u32 = 0xFE000000;
const IMM_SIGN_MASK: u32 = 0x80000000;

// Opcode constants
const LUI: u32 = 0x37;
const AUIPC: u32 = 0x17;
const JAL: u32 = 0x6f;
const JALR: u32 = 0x67;
const BRANCH: u32 = 0x63;
const LOAD: u32 = 0x03;
const STORE: u32 = 0x23;
const OP_IMM: u32 = 0x13;
const OP: u32 = 0x33;
const MEM: u32 = 0x0f;
const SYS: u32 = 0x73;

// Function constants - funct3 & funct7
const BEQ: u32 = 0x00000000;
const BNE: u32 = 0x00001000;
const BLT: u32 = 0x00004000;
const BGE: u32 = 0x00005000;
const BLTU: u32 = 0x00006000;
const BGEU: u32 = 0x00007000;
const LB: u32 = 0x00000000;
const LH: u32 = 0x00001000;
const LW: u32 = 0x00002000;
const LBU: u32 = 0x00004000;
const LHU: u32 = 0x00005000;
const SB: u32 = 0x00000000;
const SH: u32 = 0x00001000;
const SW: u32 = 0x00002000;
const ADDI: u32 = 0x00000000;
const SLTI: u32 = 0x00002000;
const SLTIU: u32 = 0x00003000;
const XORI: u32 = 0x00004000;
const ORI: u32 = 0x00006000;
const ANDI: u32 = 0x00007000;
const SLLI: u32 = 0x00001000;
const SRLI: u32 = 0x00005000;
const SRAI: u32 = 0x40005000;
const ADD: u32 = 0x00000000;
const SUB: u32 = 0x40000000;
const SLL: u32 = 0x00001000;
const SLT: u32 = 0x00002000;
const SLTU: u32 = 0x00003000;
const XOR: u32 = 0x00004000;
const SRL: u32 = 0x00005000;
const SRA: u32 = 0x40005000;
const OR: u32 = 0x00006000;
const AND: u32 = 0x00007000;
const PAUSE: u32 = 0x01000073;
const ECALL: u32 = 0x00000073;
const EBREAK: u32 = 0x00100073;
const FENCE_TSO: u32 = 0x8330000f;
const FENCE: u32 = 0x0000000f;

pub fn phrase_instruction(i_word: u32) -> Result<Instruction, String> {
    let inst = match i_word & OP_MASK {
        LUI => Instruction::Lui(UType::decode(i_word)),
        AUIPC => Instruction::Auipc(UType::decode(i_word)),
        JAL => Instruction::Jal(UType::decode_j(i_word)),
        JALR => Instruction::Jalr(IType::decode(i_word)),
        BRANCH => {
            let func = i_word & FN3_MASK;
            match func {
                BNE => Instruction::Bne(SType::decode_b(i_word)),
                BEQ => Instruction::Beq(SType::decode_b(i_word)),
                BLT => Instruction::Blt(SType::decode_b(i_word)),
                BGE => Instruction::Bge(SType::decode_b(i_word)),
                BLTU => Instruction::Bltu(SType::decode_b(i_word)),
                BGEU => Instruction::Bgeu(SType::decode_b(i_word)),
                _ => Instruction::None,
            }
        }
        LOAD => {
            let func = i_word & FN3_MASK;
            match func {
                LB => Instruction::Lb(IType::decode(i_word)),
                LH => Instruction::Lh(IType::decode(i_word)),
                LW => Instruction::Lw(IType::decode(i_word)),
                LBU => Instruction::Lbu(IType::decode(i_word)),
                LHU => Instruction::Lhu(IType::decode(i_word)),
                _ => Instruction::None,
            }
        }
        STORE => {
            let func = i_word & FN3_MASK;
            match func {
                SB => Instruction::Sb(SType::decode(i_word)),
                SH => Instruction::Sh(SType::decode(i_word)),
                SW => Instruction::Sw(SType::decode(i_word)),
                _ => Instruction::None,
            }
        }
        OP_IMM => {
            let func = i_word & FN3_MASK;
            match func {
                ADDI => Instruction::Addi(IType::decode(i_word)),
                SLTI => Instruction::Slti(IType::decode(i_word)),
                SLTIU => Instruction::Sltiu(IType::decode(i_word)),
                XORI => Instruction::Xori(IType::decode(i_word)),
                ORI => Instruction::Ori(IType::decode(i_word)),
                ANDI => Instruction::Andi(IType::decode(i_word)),
                _ => {
                    let func7 = func | (i_word & FN7_MASK);
                    match func7 {
                        SLLI => Instruction::Slli(IType::decode(i_word)),
                        SRLI => Instruction::Srli(IType::decode(i_word)),
                        SRAI => Instruction::Srai(IType::decode(i_word)),
                        _ => Instruction::None,
                    }
                }
            }
        }
        OP => {
            let func = i_word & (FN3_MASK | FN7_MASK);
            match func {
                ADD => Instruction::Add(RType::decode(i_word)),
                SUB => Instruction::Sub(RType::decode(i_word)),
                SLL => Instruction::Sll(RType::decode(i_word)),
                SLT => Instruction::Slt(RType::decode(i_word)),
                SLTU => Instruction::Sltu(RType::decode(i_word)),
                XOR => Instruction::Xor(RType::decode(i_word)),
                SRL => Instruction::Srl(RType::decode(i_word)),
                SRA => Instruction::Sra(RType::decode(i_word)),
                OR => Instruction::Or(RType::decode(i_word)),
                AND => Instruction::And(RType::decode(i_word)),
                _ => Instruction::None,
            }
        }
        MEM => match i_word {
            FENCE_TSO => Instruction::FenceTso,
            _ => Instruction::Fence(i_word),
        },
        SYS => match i_word {
            PAUSE => Instruction::Pause,
            ECALL => Instruction::Ecall,
            EBREAK => Instruction::Ebreak,
            _ => Instruction::None,
        },
        _ => Instruction::None,
    };
    match inst {
        Instruction::None => Err("Instruction not found".to_string()),
        _ => Ok(inst),
    }
}

pub struct RType {
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
}

impl RType {
    pub fn decode(inst: u32) -> Self {
        RType {
            rd: ((inst & RD_MASK) >> RD_POS) as usize,
            rs1: ((inst & RS1_MASK) >> RS1_POS) as usize,
            rs2: ((inst & RS2_MASK) >> RS2_POS) as usize,
        }
    }
    pub fn format(&self) -> String {
        let rd_alias = get_register_alias(self.rd);
        let rs1_alias = get_register_alias(self.rs1);
        let rs2_alias = get_register_alias(self.rs2);
        return format!("{}, {}, {}", rd_alias, rs1_alias, rs2_alias);
    }
}

pub struct IType {
    pub rd: usize,
    pub rs1: usize,
    pub imm: u32,
}
impl IType {
    fn decode(inst: u32) -> Self {
        IType {
            rd: ((inst & RD_MASK) >> RD_POS) as usize,
            rs1: ((inst & RS1_MASK) >> RS1_POS) as usize,
            imm: ((inst >> 20)
                | (match inst & IMM_SIGN_MASK {
                    0 => 0,
                    _ => 0xFFFFF000,
                })) as u32,
        }
    }
    pub fn format(&self) -> String {
        let rd_alias = get_register_alias(self.rd);
        let rs1_alias = get_register_alias(self.rs1);
        return format!("{}, {}, {}", rd_alias, rs1_alias, self.imm as i32);
    }
}

pub struct SType {
    pub rs1: usize,
    pub rs2: usize,
    pub imm: u32,
}
impl SType {
    fn decode(inst: u32) -> Self {
        SType {
            rs1: ((inst & RS1_MASK) >> RS1_POS) as usize,
            rs2: ((inst & RS2_MASK) >> RS2_POS) as usize,
            imm: (((inst >> 7) & 0x1F)
                | (inst >> 25)
                | (match inst & IMM_SIGN_MASK {
                    0 => 0,
                    _ => 0xFFFFF000,
                })) as u32,
        }
    }

    fn decode_b(inst: u32) -> Self {
        SType {
            rs1: ((inst & RS1_MASK) >> RS1_POS) as usize,
            rs2: ((inst & RS2_MASK) >> RS2_POS) as usize,
            imm: (((inst & 0xF00) >> 7)
                | ((inst & 0x7E000000) >> 25)
                | ((inst & 0x80) << 4)
                | (match inst & IMM_SIGN_MASK {
                    0 => 0,
                    _ => 0xFFFFF000,
                })) as u32,
        }
    }
    pub fn format(&self) -> String {
        let rs1_alias = get_register_alias(self.rs1);
        let rs2_alias = get_register_alias(self.rs2);
        return format!("{}, {}, {}", rs1_alias, rs2_alias, self.imm as i32);
    }
}

pub struct UType {
    pub rd: usize,
    pub imm: u32,
}

impl UType {

    pub fn decode(i_word: u32) -> Self {
        UType {
            rd: ((i_word & RD_MASK) >> RD_POS) as usize,
            imm: (i_word & 0xFFFFF000),
        }
    }

    pub fn decode_j(i_word: u32) -> Self {
        UType {
            rd: ((i_word & RD_MASK) >> RD_POS) as usize,
            imm: (((i_word & 0x7FE00000) >> 20)
                | ((i_word & 0x100000) >> 9)
                | (i_word & 0xFF000)
                | (match i_word & 0x80000000 {
                    0 => 0,
                    _ => 0xFFF00000,
                })) as u32,
        }
    }
    pub fn format(&self) -> String {
        let rd_alias = get_register_alias(self.rd);
        return format!("{}, {}", rd_alias, self.imm as i32);
    }
}

pub enum Instruction {
    None,
    Lui(UType),
    Auipc(UType),
    Jal(UType),
    Jalr(IType),
    Beq(SType),
    Bne(SType),
    Blt(SType),
    Bge(SType),
    Bltu(SType),
    Bgeu(SType),
    Lb(IType),
    Lh(IType),
    Lw(IType),
    Lbu(IType),
    Lhu(IType),
    Sb(SType),
    Sh(SType),
    Sw(SType),
    Addi(IType),
    Slti(IType),
    Sltiu(IType),
    Xori(IType),
    Ori(IType),
    Andi(IType),
    Slli(IType),
    Srli(IType),
    Srai(IType),
    Add(RType),
    Sub(RType),
    Sll(RType),
    Slt(RType),
    Sltu(RType),
    Xor(RType),
    Srl(RType),
    Sra(RType),
    Or(RType),
    And(RType),
    Fence(u32),
    FenceTso,
    Pause,
    Ecall,
    Ebreak,
}

impl Instruction {
    pub fn print(&self) {
        println!("{}", self.mnemonic());
    }
    pub fn mnemonic(&self) -> String {
        match self {
            Instruction::Lui(arg) => format!("lui {}", arg.format()),
            Instruction::Auipc(arg) => format!("auipc {}", arg.format()),
            Instruction::Jal(arg) => format!("jal {}", arg.format()),
            Instruction::Jalr(arg) => format!("jalr {}", arg.format()),
            Instruction::Beq(arg) => format!("beq {}", arg.format()),
            Instruction::Bne(arg) => format!("bne {}", arg.format()),
            Instruction::Blt(arg) => format!("blt {}", arg.format()),
            Instruction::Bge(arg) => format!("bge {}", arg.format()),
            Instruction::Bltu(arg) => format!("bltu {}", arg.format()),
            Instruction::Bgeu(arg) => format!("bgeu {}", arg.format()),
            Instruction::Lb(arg) => format!("lb {}", arg.format()),
            Instruction::Lh(arg) => format!("lh {}", arg.format()),
            Instruction::Lw(arg) => format!("lw {}", arg.format()),
            Instruction::Lbu(arg) => format!("lbu {}", arg.format()),
            Instruction::Lhu(arg) => format!("lhu {}", arg.format()),
            Instruction::Sb(arg) => format!("sb {}", arg.format()),
            Instruction::Sh(arg) => format!("sh {}", arg.format()),
            Instruction::Sw(arg) => format!("sw {}", arg.format()),
            Instruction::Addi(arg) => format!("addi {}", arg.format()),
            Instruction::Slti(arg) => format!("slti {}", arg.format()),
            Instruction::Sltiu(arg) => format!("sltiu {}", arg.format()),
            Instruction::Xori(arg) => format!("xori {}", arg.format()),
            Instruction::Ori(arg) => format!("ori {}", arg.format()),
            Instruction::Andi(arg) => format!("andi {}", arg.format()),
            Instruction::Slli(arg) => format!("slli {}", arg.format()),
            Instruction::Srli(arg) => format!("srli {}", arg.format()),
            Instruction::Srai(arg) => format!("srai {}", arg.format()),
            Instruction::Add(arg) => format!("add {}", arg.format()),
            Instruction::Sub(arg) => format!("sub {}", arg.format()),
            Instruction::Sll(arg) => format!("sll {}", arg.format()),
            Instruction::Slt(arg) => format!("slt {}", arg.format()),
            Instruction::Sltu(arg) => format!("sltu {}", arg.format()),
            Instruction::Xor(arg) => format!("xor {}", arg.format()),
            Instruction::Srl(arg) => format!("srl {}", arg.format()),
            Instruction::Sra(arg) => format!("sra {}", arg.format()),
            Instruction::Or(arg) => format!("or {}", arg.format()),
            Instruction::And(arg) => format!("and {}", arg.format()),
            Instruction::Fence(_) => format!("fence"),
            Instruction::FenceTso => format!("fence.tso"),
            Instruction::Pause => format!("pause"),
            Instruction::Ecall => format!("ecall"),
            Instruction::Ebreak => format!("ebreak"),
            _ => String::from(""),
        }
    }
}

pub fn get_register_alias(register: usize) -> &'static str {
    match register {
        0 => "zero",
        1 => "ra",
        2 => "sp",
        3 => "gp",
        4 => "tp",
        5 => "t0",
        6 => "t1",
        7 => "t2",
        8 => "s0",
        9 => "s1",
        10 => "a0",
        11 => "a1",
        12 => "a2",
        13 => "a3",
        14 => "a4",
        15 => "a5",
        16 => "a6",
        17 => "a7",
        18 => "s2",
        19 => "s3",
        20 => "s4",
        21 => "s5",
        22 => "s6",
        23 => "s7",
        24 => "s8",
        25 => "s9",
        26 => "s10",
        27 => "s11",
        28 => "t3",
        29 => "t4",
        30 => "t5",
        31 => "t6",
        _ => "",
    }
}
