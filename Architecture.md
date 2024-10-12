
# ISA


|31               25|24   20|19   15|14    12|11          7|6      0|        |
|-------------------|-------|-------|--------|-------------|--------|--------|
|      funct7       |  rs2  |  rs1  | funct3 |     rd      | opcode | R-type |
|       imm[11:0]           |  rs1  | funct3 |     rd      | opcode | I-type |
|     imm[11:5]     |  rs2  |  rs1  | funct3 |  imm[4:0]   | opcode | S-type |
|   imm[12|10:5]    |  rs2  |  rs1  | funct3 | imm[4:1|11] | opcode | B-type |
|                imm[31:12]                  |     rd      | opcode | U-type |
|           imm[20|10:1|11|19:12]            |     rd      | opcode | J-type |


| RV32I Base Instruction Set                                                |
| ------------------------------------------------------------------------- |
|              imm[31:12]               |     rd      | 0110111 | LUI       | U        LUI
|              imm[31:12]               |     rd      | 0010111 | AUIPC     | U        AUIPC
|         imm[20|10:1|11|19:12]         |     rd      | 1101111 | JAL       | U/J      JAL
|        imm[11:0]        |  rs1  | 000 |     rd      | 1100111 | JALR      | I        JALR
| imm[12|10:5]  |   rs2   |  rs1  | 000 | imm[4:1|11] | 1100011 | BEQ       | S/B      BRANCH    rs1 == rs2
| imm[12|10:5]  |   rs2   |  rs1  | 001 | imm[4:1|11] | 1100011 | BNE       | S/B      BRANCH    rs1 != rs2
| imm[12|10:5]  |   rs2   |  rs1  | 100 | imm[4:1|11] | 1100011 | BLT       | S/B      BRANCH    rs1 <  rs2
| imm[12|10:5]  |   rs2   |  rs1  | 101 | imm[4:1|11] | 1100011 | BGE       | S/B      BRANCH    rs1 >= rs2
| imm[12|10:5]  |   rs2   |  rs1  | 110 | imm[4:1|11] | 1100011 | BLTU      | S/B      BRANCH    rs1 <  rs2 unsigned
| imm[12|10:5]  |   rs2   |  rs1  | 111 | imm[4:1|11] | 1100011 | BGEU      | S/B      BRANCH    rs1 >= rs2 unsignes
|       imm[11:0]         |  rs1  | 000 |     rd      | 0000011 | LB        | I        LOAD      rs1 + imm -> addr   Load Byte - sign-ex to rd
|       imm[11:0]         |  rs1  | 001 |     rd      | 0000011 | LH        | I        LOAD      rs1 + imm -> addr   Load Halfword - sign-ex to rd
|       imm[11:0]         |  rs1  | 010 |     rd      | 0000011 | LW        | I        LOAD      rs1 + imm -> addr   Load Word 
|       imm[11:0]         |  rs1  | 100 |     rd      | 0000011 | LBU       | I        LOAD      rs1 + imm -> addr   Load Byte - zero ex to rd
|       imm[11:0]         |  rs1  | 101 |     rd      | 0000011 | LHU       | I        LOAD      rs1 + imm -> addr   Load Halfword - zero ex to rd
|   imm[11:5]   |   rs2   |  rs1  | 000 |  imm[4:0]   | 0100011 | SB        | S        STORE     rs1 + imm -> addr   Store byte
|   imm[11:5]   |   rs2   |  rs1  | 001 |  imm[4:0]   | 0100011 | SH        | S        STORE     rs1 + imm -> addr   Store halfword
|   imm[11:5]   |   rs2   |  rs1  | 010 |  imm[4:0]   | 0100011 | SW        | S        STORE     rs1 + imm -> addr   Store word
|        imm[11:0]        |  rs1  | 000 |     rd      | 0010011 | ADDI      | I        OP-IMM    rs1 + imm -> rd 
|        imm[11:0]        |  rs1  | 010 |     rd      | 0010011 | SLTI      | I        OP-IMM    rd = 1 if rs1 < imm else 0
|        imm[11:0]        |  rs1  | 011 |     rd      | 0010011 | SLTIU     | I        OP-IMM    rd = 1 if rs1 < imm else 0 unsignes
|        imm[11:0]        |  rs1  | 100 |     rd      | 0010011 | XORI      | I        OP-IMM    rs1 XOR imm -> rd
|        imm[11:0]        |  rs1  | 110 |     rd      | 0010011 | ORI       | I        OP-IMM    rs1 OR  imm -> rd
|        imm[11:0]        |  rs1  | 111 |     rd      | 0010011 | ANDI      | I        OP-IMM    rs1 AND imm -> rd
|    0000000    |  shamt  |  rs1  | 001 |     rd      | 0010011 | SLLI      | R        OP-IMM    rs1 << imm[4:0] logical -> rd
|    0000000    |  shamt  |  rs1  | 101 |     rd      | 0010011 | SRLI      | R        OP-IMM    rs1 << imm[4:0] logical -> rd
|    0100000    |  shamt  |  rs1  | 101 |     rd      | 0010011 | SRAI      | R        OP-IMM    rs1 << imm[4:0] arith   -> rd
|    0000000    |   rs2   |  rs1  | 000 |     rd      | 0110011 | ADD       | R        OP        rs1 + rs2 -> rd
|    0100000    |   rs2   |  rs1  | 000 |     rd      | 0110011 | SUB       | R        OP        rs1 - rs2 -> rd
|    0000000    |   rs2   |  rs1  | 001 |     rd      | 0110011 | SLL       | R        OP        rs1 << rs2[4:0] logical
|    0000000    |   rs2   |  rs1  | 010 |     rd      | 0110011 | SLT       | R        OP        rd = 1 if rs1 < rs2 else 0
|    0000000    |   rs2   |  rs1  | 011 |     rd      | 0110011 | SLTU      | R        OP        rd = 1 if rs1 < rs2 else 0 unsignes
|    0000000    |   rs2   |  rs1  | 100 |     rd      | 0110011 | XOR       | R        OP        rs1 XOR rs2
|    0000000    |   rs2   |  rs1  | 101 |     rd      | 0110011 | SRL       | R        OP        rs1 << rs2[4:0] logical
|    0100000    |   rs2   |  rs1  | 101 |     rd      | 0110011 | SRA       | R        OP        rs1 << rs2[4:0] arith
|    0000000    |   rs2   |  rs1  | 110 |     rd      | 0110011 | OR        | R        OP        rs1 OR  rs2
|    0000000    |   rs2   |  rs1  | 111 |     rd      | 0110011 | AND       | R        OP        rs1 AND rs2
|    fm     | pred | succ |  rs1  | 000 |     rd      | 0001111 | FENCE     | I        MEM
|   1000    | 0011 | 0011 | 00000 | 000 |   00000     | 0001111 | FENCE.TSO | I        MEM
|   0000    | 0001 | 0000 | 00000 | 000 |   00000     | 1110011 | PAUSE     | I        SYS
|      000000000000       | 00000 | 000 |   00000     | 1110011 | ECALL     | I        SYS
|      000000000001       | 00000 | 000 |   00000     | 1110011 | EBREAK    | I        SYS

x xxx 0110111 LUI       
x xxx 0010111 AUIPC     
x xxx 1101111 JAL      
x 000 1100111 JALR     
x 000 1100011 BEQ       rs1 == rs2
x 001 1100011 BNE       rs1 != rs2
x 100 1100011 BLT       rs1 <  rs2
x 101 1100011 BGE       rs1 >= rs2
x 110 1100011 BLTU      rs1 <  rs2 unsigned
x 111 1100011 BGEU      rs1 >= rs2 unsignes
x 000 0000011 LB        ADD rs1 + imm   Load Byte - sign-ex to rd
x 001 0000011 LH        ADD rs1 + imm   Load Halfword - sign-ex to rd
x 010 0000011 LW        ADD rs1 + imm   Load Word 
x 100 0000011 LBU       ADD rs1 + imm   Load Byte - zero ex to rd
x 101 0000011 LHU       ADD rs1 + imm   Load Halfword - zero ex to rd
x 000 0100011 SB        ADD rs1 + imm   Store byte
x 001 0100011 SH        ADD rs1 + imm   Store halfword
x 010 0100011 SW        ADD rs1 + imm   Store word
x 000 0010011 ADDI      x000 ADDI
x 010 0010011 SLTI      
x 011 0010011 SLTIU     
x 100 0010011 XORI      x100 XOR
x 110 0010011 ORI       x110 OR
x 111 0010011 ANDI      x111 AND
0 001 0010011 SLLI      
0 101 0010011 SRLI      
1 101 0010011 SRAI      
0 000 0110011 ADD       0000 ADD
1 000 0110011 SUB       1000 SUB
0 001 0110011 SLL       
0 010 0110011 SLT       
0 011 0110011 SLTU      
0 100 0110011 XOR       0100 XOR
0 101 0110011 SRL       
1 101 0110011 SRA       
0 110 0110011 OR        0110 OR
0 111 0110011 AND       0111 AND

01101 U
00101 U
11011 U/J
11001 I
11000 S/B
00000 I
01000 S
00100 I
01100 R
00011 
11100


# Memory

Capacity = 4096 Bytes
Memory blocks of u32 = 4096 Bytes / 4 = 1024

            |             word              | 
0xFFFFFFFC  | byte0 | byte1 | byte2 | byte3 |
            ...
0x0000000C  | byte0 | byte1 | byte2 | byte3 |
0x00000008  | byte0 | byte1 | byte2 | byte3 |
0x00000004  | byte0 | byte1 | byte2 | byte3 | 
0x00000000  | byte0 | byte1 | byte2 | byte3 | 

Little-endian - byte 0 holds the MSB and byte 3 holds LSB




