
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
|              imm[31:12]               |     rd      | 0110111 | LUI       |
|              imm[31:12]               |     rd      | 0010111 | AUIPC     |
|         imm[20|10:1|11|19:12]         |     rd      | 1101111 | JAL       |
|        imm[11:0]        |  rs1  | 000 |     rd      | 1100111 | JALR      |
| imm[12|10:5]  |   rs2   |  rs1  | 000 | imm[4:1|11] | 1100011 | BEQ       |
| imm[12|10:5]  |   rs2   |  rs1  | 001 | imm[4:1|11] | 1100011 | BNE       |
| imm[12|10:5]  |   rs2   |  rs1  | 100 | imm[4:1|11] | 1100011 | BLT       |
| imm[12|10:5]  |   rs2   |  rs1  | 101 | imm[4:1|11] | 1100011 | BGE       |
| imm[12|10:5]  |   rs2   |  rs1  | 110 | imm[4:1|11] | 1100011 | BLTU      |
| imm[12|10:5]  |   rs2   |  rs1  | 111 | imm[4:1|11] | 1100011 | BGEU      |
|       imm[11:0]         |  rs1  | 000 |     rd      | 0000011 | LB        |
|       imm[11:0]         |  rs1  | 001 |     rd      | 0000011 | LH        |
|       imm[11:0]         |  rs1  | 010 |     rd      | 0000011 | LW        |
|       imm[11:0]         |  rs1  | 100 |     rd      | 0000011 | LBU       |
|       imm[11:0]         |  rs1  | 101 |     rd      | 0000011 | LHU       |
|   imm[11:5]   |   rs2   |  rs1  | 000 |  imm[4:0]   | 0100011 | SB        |
|   imm[11:5]   |   rs2   |  rs1  | 001 |  imm[4:0]   | 0100011 | SH        |
|   imm[11:5]   |   rs2   |  rs1  | 010 |  imm[4:0]   | 0100011 | SW        |
|        imm[11:0]        |  rs1  | 000 |     rd      | 0010011 | ADDI      |
|        imm[11:0]        |  rs1  | 010 |     rd      | 0010011 | SLTI      |
|        imm[11:0]        |  rs1  | 011 |     rd      | 0010011 | SLTIU     |
|        imm[11:0]        |  rs1  | 100 |     rd      | 0010011 | XORI      |
|        imm[11:0]        |  rs1  | 110 |     rd      | 0010011 | ORI       |
|        imm[11:0]        |  rs1  | 111 |     rd      | 0010011 | ANDI      |
|    0000000    |  shamt  |  rs1  | 001 |     rd      | 0010011 | SLLI      |
|    0000000    |  shamt  |  rs1  | 101 |     rd      | 0010011 | SRLI      |
|    0100000    |  shamt  |  rs1  | 101 |     rd      | 0010011 | SRAI      |
|    0000000    |   rs2   |  rs1  | 000 |     rd      | 0110011 | ADD       |
|    0100000    |   rs2   |  rs1  | 000 |     rd      | 0110011 | SUB       |
|    0000000    |   rs2   |  rs1  | 001 |     rd      | 0110011 | SLL       |
|    0000000    |   rs2   |  rs1  | 010 |     rd      | 0110011 | SLT       |
|    0000000    |   rs2   |  rs1  | 011 |     rd      | 0110011 | SLTU      |
|    0000000    |   rs2   |  rs1  | 100 |     rd      | 0110011 | XOR       |
|    0000000    |   rs2   |  rs1  | 101 |     rd      | 0110011 | SRL       |
|    0100000    |   rs2   |  rs1  | 101 |     rd      | 0110011 | SRA       |
|    0000000    |   rs2   |  rs1  | 110 |     rd      | 0110011 | OR        |
|    0000000    |   rs2   |  rs1  | 111 |     rd      | 0110011 | AND       |
|    fm     | pred | succ |  rs1  | 000 |     rd      | 0001111 | FENCE     |
|   1000    | 0011 | 0011 | 00000 | 000 |   00000     | 0001111 | FENCE.TSO |
|   0000    | 0001 | 0000 | 00000 | 000 |   00000     | 1110011 | PAUSE     |
|      000000000000       | 00000 | 000 |   00000     | 1110011 | ECALL     |
|      000000000001       | 00000 | 000 |   00000     | 1110011 | EBREAK    |




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




