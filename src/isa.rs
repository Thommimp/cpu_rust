const FUNCT3_MASK: u32 = 0x00007000;
const FUNCT7_MASK: u32 = 0xFE000000;

const RD_POS: usize = 7;
const RS1_POS: usize = 15;
const RS2_POS: usize = 20;
const FUNCT3_POS: usize = 12;
const FUNCT7_POS: usize = 25;
const OP_MASK: u32 = 0x0000007F;
const RD_MASK: u32 = 0x00000F80;
const RS1_MASK: u32 = 0x000F8000;
const RS2_MASK: u32 = 0x01F00000;
const FN3_MASK: u32 = 0x00007000;
const FN7_MASK: u32 = 0xFE000000;
const IMM_SIGN_MASK: u32 = 0x80000000;
pub const R_TYPE_MASK: u32 = OP_MASK | FN3_MASK | FN7_MASK;
pub const I_TYPE_MASK: u32 = OP_MASK | FN3_MASK;
pub const S_TYPE_MASK: u32 = OP_MASK | FN3_MASK;
pub const U_TYPE_MASK: u32 = OP_MASK;


#[derive(Clone)]
pub struct Instruction {
    pub opcode: u32,
    pub mask: u32,
    pub name: &'static str,
    pub decode: fn(i_word: u32) -> InstructionArg,
    pub ex_src: ExSrc,
    pub execute: fn(op_1: u32, op_2: u32) -> u32,
    pub wb_src: WBSrc,
}

#[derive(Clone)]
pub enum ExSrc {
    Reg2,
    Imm,
    None,
}
#[derive(Clone)]
pub enum WBSrc {
    Memory,
    Result,
    None,
}

pub fn phrase_instruction(i_word: u32) -> Result<Instruction, String> {

    for (index, value) in INSTRUCTIONS.iter().enumerate() {
        if (i_word & INSTRUCTIONS[index].mask) == INSTRUCTIONS[index].opcode {
            return Ok((*value).clone());
        }
    }
    Err(String::from("Instruction not found"))
}

pub const INSTRUCTIONS: &[Instruction] = &[
    Instruction {
        opcode: 0x00000037,
        mask: U_TYPE_MASK,
        name: "lui",
        decode: decode_u_type,
        ex_src: ExSrc::None,
        execute: op_add,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x00000017,
        mask: U_TYPE_MASK,
        name: "auipc",
        decode: decode_u_type,
        ex_src: ExSrc::None,
        execute: op_add,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x0000006F,
        mask: U_TYPE_MASK,
        name: "jal",
        decode: decode_j_type,
        ex_src: ExSrc::None,
        execute: op_add,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x00000067,
        mask: I_TYPE_MASK,
        name: "jalr",
        decode: decode_i_type,
        ex_src: ExSrc::None,
        execute: op_add,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x00000063,
        mask: S_TYPE_MASK,
        name: "beq",
        decode: decode_b_type,
        ex_src: ExSrc::Reg2,
        execute: op_beq,
        wb_src: WBSrc::None,
    },
    Instruction {
        opcode: 0x00001063,
        mask: S_TYPE_MASK,
        name: "bne",
        decode: decode_b_type,
        ex_src: ExSrc::Reg2,
        execute: op_bne,
        wb_src: WBSrc::None,
    },
    Instruction {
        opcode: 0x00004063,
        mask: S_TYPE_MASK,
        name: "blt",
        decode: decode_b_type,
        ex_src: ExSrc::Reg2,
        execute: op_blt,
        wb_src: WBSrc::None,
    },
    Instruction {
        opcode: 0x00005063,
        mask: S_TYPE_MASK,
        name: "bge",
        decode: decode_b_type,
        ex_src: ExSrc::Reg2,
        execute: op_bge,
        wb_src: WBSrc::None,
    },
    Instruction {
        opcode: 0x00006063,
        mask: S_TYPE_MASK,
        name: "bltu",
        decode: decode_b_type,
        ex_src: ExSrc::Reg2,
        execute: op_bltu,
        wb_src: WBSrc::None,
    },
    Instruction {
        opcode: 0x00007063,
        mask: S_TYPE_MASK,
        name: "bgeu",
        decode: decode_b_type,
        ex_src: ExSrc::Reg2,
        execute: op_bgeu,
        wb_src: WBSrc::None,
    },
    Instruction {
        opcode: 0x00000003,
        mask: I_TYPE_MASK,
        name: "lb",
        decode: decode_i_type,
        ex_src: ExSrc::Imm,
        execute: op_add,
        wb_src: WBSrc::Memory,
    },
    Instruction {
        opcode: 0x00001003,
        mask: I_TYPE_MASK,
        name: "lh",
        decode: decode_i_type,
        ex_src: ExSrc::Imm,
        execute: op_add,
        wb_src: WBSrc::Memory,
    },
    Instruction {
        opcode: 0x00002003,
        mask: I_TYPE_MASK,
        name: "lw",
        decode: decode_i_type,
        ex_src: ExSrc::Imm,
        execute: op_add,
        wb_src: WBSrc::Memory,
    },
    Instruction {
        opcode: 0x00004003,
        mask: I_TYPE_MASK,
        name: "lbu",
        decode: decode_i_type,
        ex_src: ExSrc::Imm,
        execute: op_add,
        wb_src: WBSrc::Memory,
    },
    Instruction {
        opcode: 0x00005003,
        mask: I_TYPE_MASK,
        name: "lhu",
        decode: decode_i_type,
        ex_src: ExSrc::Imm,
        execute: op_add,
        wb_src: WBSrc::Memory,
    },
    Instruction {
        opcode: 0x00000023,
        mask: S_TYPE_MASK,
        name: "sb",
        decode: decode_s_type,
        ex_src: ExSrc::Imm,
        execute: op_add,
        wb_src: WBSrc::None,
    },
    Instruction {
        opcode: 0x00001023,
        mask: S_TYPE_MASK,
        name: "sh",
        decode: decode_s_type,
        ex_src: ExSrc::Imm,
        execute: op_add,
        wb_src: WBSrc::None,
    },
    Instruction {
        opcode: 0x00002023,
        mask: S_TYPE_MASK,
        name: "sw",
        decode: decode_s_type,
        ex_src: ExSrc::Imm,
        execute: op_add,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x00000013,
        mask: I_TYPE_MASK,
        name: "addi",
        decode: decode_i_type,
        ex_src: ExSrc::Imm,
        execute: op_add,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x00002013,
        mask: I_TYPE_MASK,
        name: "slti",
        decode: decode_i_type,
        ex_src: ExSrc::Imm,
        execute: op_slt,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x00003013,
        mask: I_TYPE_MASK,
        name: "sltiu",
        decode: decode_i_type,
        ex_src: ExSrc::Imm,
        execute: op_sltu,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x00004013,
        mask: I_TYPE_MASK,
        name: "xori",
        decode: decode_i_type,
        ex_src: ExSrc::Imm,
        execute: op_xor,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x00006013,
        mask: I_TYPE_MASK,
        name: "ori",
        decode: decode_i_type,
        ex_src: ExSrc::Imm,
        execute: op_or,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x00007013,
        mask: I_TYPE_MASK,
        name: "andi",
        decode: decode_i_type,
        ex_src: ExSrc::Imm,
        execute: op_and,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x00001013,
        mask: R_TYPE_MASK,
        name: "slli",
        decode: decode_r_type,
        ex_src: ExSrc::Imm,
        execute: op_sll,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x00005013,
        mask: R_TYPE_MASK,
        name: "srli",
        decode: decode_r_type,
        ex_src: ExSrc::Imm,
        execute: op_srl,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x40005013,
        mask: R_TYPE_MASK,
        name: "srai",
        decode: decode_r_type,
        ex_src: ExSrc::Imm,
        execute: op_sra,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x00000033,
        mask: R_TYPE_MASK,
        name: "add",
        decode: decode_r_type,
        ex_src: ExSrc::Reg2,
        execute: op_add,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x40000033,
        mask: R_TYPE_MASK,
        name: "sub",
        decode: decode_r_type,
        ex_src: ExSrc::Reg2,
        execute: op_sub,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x00001033,
        mask: R_TYPE_MASK,
        name: "sll",
        decode: decode_r_type,
        ex_src: ExSrc::Reg2,
        execute: op_sll,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x00002033,
        mask: R_TYPE_MASK,
        name: "slt",
        decode: decode_r_type,
        ex_src: ExSrc::Reg2,
        execute: op_slt,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x00003033,
        mask: R_TYPE_MASK,
        name: "sltu",
        decode: decode_r_type,
        ex_src: ExSrc::Reg2,
        execute: op_slt,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x00004033,
        mask: R_TYPE_MASK,
        name: "xor",
        decode: decode_r_type,
        ex_src: ExSrc::Reg2,
        execute: op_xor,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x00005033,
        mask: R_TYPE_MASK,
        name: "srl",
        decode: decode_r_type,
        ex_src: ExSrc::Reg2,
        execute: op_srl,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x40005033,
        mask: R_TYPE_MASK,
        name: "sra",
        decode: decode_r_type,
        ex_src: ExSrc::Reg2,
        execute: op_sra,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x00006033,
        mask: R_TYPE_MASK,
        name: "or",
        decode: decode_r_type,
        ex_src: ExSrc::Reg2,
        execute: op_or,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x00007033,
        mask: R_TYPE_MASK,
        name: "and",
        decode: decode_r_type,
        ex_src: ExSrc::Reg2,
        execute: op_and,
        wb_src: WBSrc::Result,
    },
    Instruction {
        opcode: 0x0000000F,
        mask: I_TYPE_MASK,
        name: "fence",
        decode: decode_i_type,
        ex_src: ExSrc::None,
        execute: op_add,
        wb_src: WBSrc::None,
    },
    Instruction {
        opcode: 0x8330000F,
        mask: I_TYPE_MASK,
        name: "fence.tso",
        decode: decode_i_type,
        ex_src: ExSrc::None,
        execute: op_add,
        wb_src: WBSrc::None,
    },
    Instruction {
        opcode: 0x01000073,
        mask: I_TYPE_MASK,
        name: "pause",
        decode: decode_i_type,
        ex_src: ExSrc::None,
        execute: op_add,
        wb_src: WBSrc::None,
    },
    Instruction {
        opcode: 0x00000073,
        mask: I_TYPE_MASK,
        name: "ecall",
        decode: decode_i_type,
        ex_src: ExSrc::None,
        execute: op_add,
        wb_src: WBSrc::None,
    },
    Instruction {
        opcode: 0x00100073,
        mask: I_TYPE_MASK,
        name: "ebreak",
        decode: decode_i_type,
        ex_src: ExSrc::None,
        execute: op_add,
        wb_src: WBSrc::None,
    },
];

pub fn op_add(op_1: u32, op_2: u32) -> u32 {
    (op_1 as i32 + op_2 as i32) as u32
}
pub fn op_sub(op_1: u32, op_2: u32) -> u32 {
    (op_1 as i32 + op_2 as i32) as u32
}
pub fn op_beq(op_1: u32, op_2: u32) -> u32 {
    !(op_1 == op_2) as u32
}
pub fn op_bne(op_1: u32, op_2: u32) -> u32 {
    !(op_1 != op_2) as u32
}
pub fn op_blt(op_1: u32, op_2: u32) -> u32 {
    !((op_1 as i32) < (op_2 as i32)) as u32
}
pub fn op_bge(op_1: u32, op_2: u32) -> u32 {
    !((op_1 as i32) >= (op_2 as i32)) as u32
}
pub fn op_bltu(op_1: u32, op_2: u32) -> u32 {
    !(op_1 < op_2) as u32
}
pub fn op_bgeu(op_1: u32, op_2: u32) -> u32 {
    !(op_1 >= op_2) as u32
}
pub fn op_slt(op_1: u32, op_2: u32) -> u32 {
    ((op_1 as i32) < (op_2 as i32)) as u32
}
pub fn op_sltu(op_1: u32, op_2: u32) -> u32 {
    (op_1 < op_2) as u32
}
pub fn op_xor(op_1: u32, op_2: u32) -> u32 {
    op_1 ^ op_2
}
pub fn op_or(op_1: u32, op_2: u32) -> u32 {
    op_1 | op_2
}
pub fn op_and(op_1: u32, op_2: u32) -> u32 {
    op_1 & op_2
}
pub fn op_sll(op_1: u32, op_2: u32) -> u32 {
    op_1 << (op_2 & 0x1F)
}
pub fn op_srl(op_1: u32, op_2: u32) -> u32 {
    op_1 >> (op_2 & 0x1F)
}
pub fn op_sra(op_1: u32, op_2: u32) -> u32 {
    ((op_1 as i32) >> (op_2 & 0x1F)) as u32
}

pub struct InstructionArg {
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
    pub imm: u32,
}

fn decode_r_type(inst: u32) -> InstructionArg {
    InstructionArg {
        rd: ((inst & RD_MASK) >> RD_POS) as usize,
        rs1: ((inst & RS1_MASK) >> RS1_POS) as usize,
        rs2: ((inst & RS2_MASK) >> RS2_POS) as usize,
        imm: 0,
    }
}

fn decode_i_type(inst: u32) -> InstructionArg {
    InstructionArg {
        rd: ((inst & RD_MASK) >> RD_POS) as usize,
        rs1: ((inst & RS1_MASK) >> RS1_POS) as usize,
        rs2: 0,
        imm: ((inst >> 20)
            | (match inst & IMM_SIGN_MASK {
                0 => 0,
                _ => 0xFFFFF000,
            })) as u32,
    }
}

fn decode_s_type(inst: u32) -> InstructionArg {
    InstructionArg {
        rd: 0,
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

fn decode_b_type(inst: u32) -> InstructionArg {
    InstructionArg {
        rd: 0,
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

fn decode_u_type(inst: u32) -> InstructionArg {
    InstructionArg {
        rd: ((inst & RD_MASK) >> RD_POS) as usize,
        rs1: 0,
        rs2: 0,
        imm: (inst & 0xFFFFF000),
    }
}

fn decode_j_type(inst: u32) -> InstructionArg {
    InstructionArg {
        rd: ((inst & RD_MASK) >> RD_POS) as usize,
        rs1: 0,
        rs2: 0,
        imm: (((inst & 0x7FE00000) >> 20)
            | ((inst & 0x100000) >> 9)
            | (inst & 0xFF000)
            | (match inst & 0x80000000 {
                0 => 0,
                _ => 0xFFF00000,
            })) as u32,
    }
}

enum Instructions {
    Lui,
    Auipc,
    Jal,
    Jalr,
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
    Sb,
    Sh,
    Sw,
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
    Slli,
    Srli,
    Srai,
    Add,
    Sub,
    Sll,
    Slt,
    Sltu,
    Xor,
    Srl,
    Sra,
    Or,
    And,
    Fence,
    FenceTso,
    Pause,
    Ecall,
    EbreaK,
}

