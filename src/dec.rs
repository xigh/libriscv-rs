use crate::{Inst, Gpr, ByteReader};

#[allow(dead_code)]
pub fn decode(bytes: &mut dyn ByteReader, is64bits: bool) -> Inst {
    let b0 = match bytes.next() {
        None => return Inst::ERROR,
        Some(b) => b,
    };

    match b0 & 0b11 {
        0b11 => decode_quadrant3(bytes, b0, is64bits),
        _ => Inst::TODO,
    }
}

fn decode_quadrant3(bytes: &mut dyn ByteReader, b0: u8, is64bits: bool) -> Inst {
    let mut w = b0 as u32;
    for n in 1..4 {
        let wn = match bytes.next() {
            None => return Inst::ERROR,
            Some(b) => b,
        };
        w |= (wn as u32) << n * 8;
    }

    match op_bits(w) {
        0b00000 => match funct3_bits(w) {
            0b000 => Inst::LB(rd_bits(w), rs1_bits(w), imm11_bits(w)),
            0b001 => Inst::LH(rd_bits(w), rs1_bits(w), imm11_bits(w)),
            0b010 => Inst::LW(rd_bits(w), rs1_bits(w), imm11_bits(w)),
            0b011 => Inst::LD(rd_bits(w), rs1_bits(w), imm11_bits(w)),
            0b100 => Inst::LBU(rd_bits(w), rs1_bits(w), imm11_bits(w)),
            0b101 => Inst::LHU(rd_bits(w), rs1_bits(w), imm11_bits(w)),
            0b110 => Inst::LWU(rd_bits(w), rs1_bits(w), imm11_bits(w)),
            _ => Inst::UNDEF(w),
        },
        0b01000 => match funct3_bits(w) {
            0b000 => Inst::SB(rd_bits(w), rs1_bits(w), 0),
            0b001 => Inst::SH(rd_bits(w), rs1_bits(w), 0),
            0b010 => Inst::SW(rd_bits(w), rs1_bits(w), 0),
            0b011 => Inst::SD(rd_bits(w), rs1_bits(w), 0),
            _ => Inst::UNDEF(w),
        },
        0b01100 => match funct3_bits(w) {
            0b000 => match funct7_bits(w) {
                0x00 => Inst::ADD(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                0x01 => Inst::MUL(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                0x20 => Inst::SUB(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                _ => Inst::UNDEF(w),
            },
            0b001 => match funct7_bits(w) {
                0x00 => Inst::SLL(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                0x01 => Inst::MULH(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                _ => Inst::UNDEF(w),
            },
            0b010 => match funct7_bits(w) {
                0x00 => Inst::SLT(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                0x01 => Inst::MULSU(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                _ => Inst::UNDEF(w),
            },
            0b011 => match funct7_bits(w) {
                0x00 => Inst::SLTU(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                0x01 => Inst::MULU(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                _ => Inst::UNDEF(w),
            },
            0b100 => match funct7_bits(w) {
                0x00 => Inst::XOR(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                0x01 => Inst::DIV(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                _ => Inst::UNDEF(w),
            },
            0b101 => match funct7_bits(w) {
                0x00 => Inst::SRL(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                0x01 => Inst::DIVU(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                0x20 => Inst::SRA(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                _ => Inst::UNDEF(w),
            },
            0b110 => match funct7_bits(w) {
                0x00 => Inst::OR(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                0x01 => Inst::REM(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                _ => Inst::UNDEF(w),
            },
            0b111 => match funct7_bits(w) {
                0x00 => Inst::AND(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                0x01 => Inst::REMU(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                _ => Inst::UNDEF(w),
            },
            _ => unreachable!(),
        },
        0b00100 => {
            match funct3_bits(w) {
                0b000 => Inst::ADDI(rd_bits(w), rs1_bits(w), imm11_bits(w)),
                0b001 => if is64bits {
                    match funct6_bits(w) {
                        00 => Inst::SLLI(rd_bits(w), rs1_bits(w), shamt64_bits(w)),
                        _ => Inst::UNDEF(w),
                    }
                } else {
                    match funct7_bits(w) {
                        00 => Inst::SLLI(rd_bits(w), rs1_bits(w), shamt32_bits(w)),
                        _ => Inst::UNDEF(w),
                    }
                }
                0b010 => Inst::SLTI(rd_bits(w), rs1_bits(w), imm11_bits(w)),
                0b011 => Inst::SLTUI(rd_bits(w), rs1_bits(w), imm11_bits(w)),
                0b100 => Inst::XORI(rd_bits(w), rs1_bits(w), imm11_bits(w)),
                0b101 => if is64bits {
                    match funct6_bits(w) {
                        0x00 => Inst::SRLI(rd_bits(w), rs1_bits(w), shamt64_bits(w)),
                        0x10 => Inst::SRAI(rd_bits(w), rs1_bits(w), shamt64_bits(w)),
                        _ => Inst::UNDEF(w),
                    }
                } else {
                    match funct7_bits(w) {
                        0x00 => Inst::SRLI(rd_bits(w), rs1_bits(w), shamt32_bits(w)),
                        0x20 => Inst::SRAI(rd_bits(w), rs1_bits(w), shamt32_bits(w)),
                        _ => Inst::UNDEF(w),
                    }
                },
                0b110 => Inst::ORI(rd_bits(w), rs1_bits(w), imm11_bits(w)),
                0b111 => Inst::ANDI(rd_bits(w), rs1_bits(w), imm11_bits(w)),
                _ => unreachable!(),
            }
        },
        0b11000 => match funct3_bits(w) {
            0b000 => Inst::BEQ(rs1_bits(w), rs2_bits(w), bimm_bits(w)),
            0b001 => Inst::BNE(rs1_bits(w), rs2_bits(w), bimm_bits(w)),
            0b100 => Inst::BLT(rs1_bits(w), rs2_bits(w), bimm_bits(w)),
            0b101 => Inst::BGE(rs1_bits(w), rs2_bits(w), bimm_bits(w)),
            0b110 => Inst::BLTU(rs1_bits(w), rs2_bits(w), bimm_bits(w)),
            0b111 => Inst::BGEU(rs1_bits(w), rs2_bits(w), bimm_bits(w)),
            _ => Inst::UNDEF(w),
        },
        0b01110 => match funct3_bits(w) {
            0b000 => match funct7_bits(w) {
                0x00 => Inst::ADDW(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                0x01 => Inst::MULW(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                0x20 => Inst::SUBW(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                _ => Inst::UNDEF(w),
            },
            0b001 => match funct7_bits(w) {
                0x00 => Inst::SLL(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                _ => Inst::UNDEF(w),
            },
            0b100 => match funct7_bits(w) {
                0x01 => Inst::DIVW(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                _ => Inst::UNDEF(w),
            },
            0b101 => match funct7_bits(w) {
                0x00 => Inst::SRLW(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                0x01 => Inst::DIVUW(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                0x20 => Inst::SRAW(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                _ => Inst::UNDEF(w),
            },
            0b110 => match funct7_bits(w) {
                0x01 => Inst::REMW(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                _ => Inst::UNDEF(w),
            },
            0b111 => match funct7_bits(w) {
                0x01 => Inst::REMUW(rd_bits(w), rs1_bits(w), rs2_bits(w)),
                _ => Inst::UNDEF(w),
            },
            _ => Inst::UNDEF(w),
        }
        0b11011 => Inst::JAL(rd_bits(w), 0),
        0b11001 => Inst::JALR(rd_bits(w), rs1_bits(w), 0),
        0b01101 => Inst::LUI(rd_bits(w), imm20_bits(w)),
        0b00101 => Inst::AUIPC(rd_bits(w), imm20_bits(w)),
        0b00110 => match funct3_bits(w) {
            0x00 => Inst::ADDIW(rd_bits(w), rs1_bits(w), imm11_bits(w)),
            0x01 => Inst::SLLIW(rd_bits(w), rs1_bits(w), shamt32_bits(w)),
            0x05 => match funct7_bits(w) {
                0x00 => Inst::SRLIW(rd_bits(w), rs1_bits(w), shamt32_bits(w)),
                0x20 => Inst::SRAIW(rd_bits(w), rs1_bits(w), shamt32_bits(w)),
                _ => Inst::UNDEF(w),
            }
            _ => Inst::UNDEF(w),
        },
        0b11100 => match funct3_bits(w) {
            0x00 => match funct7_bits(w) {
                0x00 => Inst::ECALL,
                0x01 => Inst::EBREAK,
                _ => Inst::UNDEF(w),
            },
            _ => Inst::UNDEF(w),
        }
        _ => Inst::UNDEF(w),
    }
}


fn op_bits(w: u32) -> u32 {
    (w >> 2) & 0b11111
}

fn funct3_bits(w: u32) -> u32 {
    (w >> 12) & 0b111
}

fn funct7_bits(w: u32) -> u32 {
    (w >> 25) & 0b1111111
}

fn funct6_bits(w: u32) -> u32 {
    (w >> 26) & 0b111111
}

fn rd_bits(w: u32) -> Gpr {
    Gpr::from_u8(((w >> 7) & 0b11111) as u8).unwrap()
}

fn rs1_bits(w: u32) -> Gpr {
    Gpr::from_u8(((w >> 15) & 0b11111) as u8).unwrap()
}

fn rs2_bits(w: u32) -> Gpr {
    Gpr::from_u8(((w >> 20) & 0b11111) as u8).unwrap()
}

fn imm11_bits(w: u32) -> u32 {
    (w >> 20) & 0b111111111111
}

fn shamt32_bits(w: u32) -> u32 {
    (w >> 20) & 0b11111
}

fn shamt64_bits(w: u32) -> u32 {
    (w >> 20) & 0b111111
}

fn imm20_bits(w: u32) -> u32 {
    w & 0b11111111111111111111000000000000
}

fn bimm_bits(w: u32) -> u32 {
    let b12 = (w >> 31) & 1;
    let b10_5 = (w >> 25) & 0b111111;
    let b4_1 = (w >> 8) & 0b1111;
    let b11 = (w >> 7) & 1;
    return b12 << 12 | b11 << 11 | b10_5 << 5 | b4_1 << 1;
}
