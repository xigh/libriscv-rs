use crate::{ByteReader, Gpr, Csr, Inst};

#[allow(dead_code)]
pub fn decode(bytes: &mut dyn ByteReader, is64bits: bool) -> Inst {
    let b0 = match bytes.next() {
        None => return Inst::ERROR,
        Some(b) => b,
    };

    match b0 & 0b11 {
        0b11 => decode_quadrant3(bytes, b0, is64bits),
        _ => {
            panic!("quadrant {} not implemented", b0 & 0b11);
        },
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
            0b000 => Inst::LB(rd_gpr(w), rs1_gpr(w), imm11_bits(w)),
            0b001 => Inst::LH(rd_gpr(w), rs1_gpr(w), imm11_bits(w)),
            0b010 => Inst::LW(rd_gpr(w), rs1_gpr(w), imm11_bits(w)),
            0b011 => Inst::LD(rd_gpr(w), rs1_gpr(w), imm11_bits(w)),
            0b100 => Inst::LBU(rd_gpr(w), rs1_gpr(w), imm11_bits(w)),
            0b101 => Inst::LHU(rd_gpr(w), rs1_gpr(w), imm11_bits(w)),
            0b110 => Inst::LWU(rd_gpr(w), rs1_gpr(w), imm11_bits(w)),
            _ => Inst::UNDEF(w),
        },
        0b00011 => match funct3_bits(w) {
            0b001 => Inst::FENCEI(rd_gpr(w), rs1_gpr(w), imm11_bits(w)),
            _ => Inst::UNDEF(w),
        },
        0b00100 => match funct3_bits(w) {
            0b000 => Inst::ADDI(rd_gpr(w), rs1_gpr(w), imm11_bits(w)),
            0b001 => {
                if is64bits {
                    match funct6_bits(w) {
                        00 => Inst::SLLI(rd_gpr(w), rs1_gpr(w), shamt64_bits(w)),
                        _ => Inst::UNDEF(w),
                    }
                } else {
                    match funct7_bits(w) {
                        00 => Inst::SLLI(rd_gpr(w), rs1_gpr(w), shamt32_bits(w)),
                        _ => Inst::UNDEF(w),
                    }
                }
            }
            0b010 => Inst::SLTI(rd_gpr(w), rs1_gpr(w), imm11_bits(w)),
            0b011 => Inst::SLTUI(rd_gpr(w), rs1_gpr(w), imm11_bits(w)),
            0b100 => Inst::XORI(rd_gpr(w), rs1_gpr(w), imm11_bits(w)),
            0b101 => {
                if is64bits {
                    match funct6_bits(w) {
                        0x00 => Inst::SRLI(rd_gpr(w), rs1_gpr(w), shamt64_bits(w)),
                        0x10 => Inst::SRAI(rd_gpr(w), rs1_gpr(w), shamt64_bits(w)),
                        _ => Inst::UNDEF(w),
                    }
                } else {
                    match funct7_bits(w) {
                        0x00 => Inst::SRLI(rd_gpr(w), rs1_gpr(w), shamt32_bits(w)),
                        0x20 => Inst::SRAI(rd_gpr(w), rs1_gpr(w), shamt32_bits(w)),
                        _ => Inst::UNDEF(w),
                    }
                }
            }
            0b110 => Inst::ORI(rd_gpr(w), rs1_gpr(w), imm11_bits(w)),
            0b111 => Inst::ANDI(rd_gpr(w), rs1_gpr(w), imm11_bits(w)),
            _ => unreachable!(),
        },
        0b00101 => Inst::AUIPC(rd_gpr(w), imm20_bits(w)),
        0b00110 => match funct3_bits(w) {
            0x00 => Inst::ADDIW(rd_gpr(w), rs1_gpr(w), imm11_bits(w)),
            0x01 => Inst::SLLIW(rd_gpr(w), rs1_gpr(w), shamt32_bits(w)),
            0x05 => match funct7_bits(w) {
                0x00 => Inst::SRLIW(rd_gpr(w), rs1_gpr(w), shamt32_bits(w)),
                0x20 => Inst::SRAIW(rd_gpr(w), rs1_gpr(w), shamt32_bits(w)),
                _ => Inst::UNDEF(w),
            },
            _ => Inst::UNDEF(w),
        },
        0b01000 => match funct3_bits(w) {
            0b000 => Inst::SB(rd_gpr(w), rs1_gpr(w), 0),
            0b001 => Inst::SH(rd_gpr(w), rs1_gpr(w), 0),
            0b010 => Inst::SW(rd_gpr(w), rs1_gpr(w), 0),
            0b011 => Inst::SD(rd_gpr(w), rs1_gpr(w), 0),
            _ => Inst::UNDEF(w),
        },
        0b01100 => match funct3_bits(w) {
            0b000 => match funct7_bits(w) {
                0x00 => Inst::ADD(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0x01 => Inst::MUL(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0x20 => Inst::SUB(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                _ => Inst::UNDEF(w),
            },
            0b001 => match funct7_bits(w) {
                0x00 => Inst::SLL(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0x01 => Inst::MULH(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                _ => Inst::UNDEF(w),
            },
            0b010 => match funct7_bits(w) {
                0x00 => Inst::SLT(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0x01 => Inst::MULSU(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                _ => Inst::UNDEF(w),
            },
            0b011 => match funct7_bits(w) {
                0x00 => Inst::SLTU(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0x01 => Inst::MULU(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                _ => Inst::UNDEF(w),
            },
            0b100 => match funct7_bits(w) {
                0x00 => Inst::XOR(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0x01 => Inst::DIV(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                _ => Inst::UNDEF(w),
            },
            0b101 => match funct7_bits(w) {
                0x00 => Inst::SRL(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0x01 => Inst::DIVU(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0x20 => Inst::SRA(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                _ => Inst::UNDEF(w),
            },
            0b110 => match funct7_bits(w) {
                0x00 => Inst::OR(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0x01 => Inst::REM(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                _ => Inst::UNDEF(w),
            },
            0b111 => match funct7_bits(w) {
                0x00 => Inst::AND(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0x01 => Inst::REMU(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                _ => Inst::UNDEF(w),
            },
            _ => unreachable!(),
        },
        0b01101 => Inst::LUI(rd_gpr(w), imm20_bits(w)),
        0b01110 => match funct3_bits(w) {
            0b000 => match funct7_bits(w) {
                0x00 => Inst::ADDW(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0x01 => Inst::MULW(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0x20 => Inst::SUBW(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                _ => Inst::UNDEF(w),
            },
            0b001 => match funct7_bits(w) {
                0x00 => Inst::SLL(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                _ => Inst::UNDEF(w),
            },
            0b100 => match funct7_bits(w) {
                0x01 => Inst::DIVW(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                _ => Inst::UNDEF(w),
            },
            0b101 => match funct7_bits(w) {
                0x00 => Inst::SRLW(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0x01 => Inst::DIVUW(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0x20 => Inst::SRAW(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                _ => Inst::UNDEF(w),
            },
            0b110 => match funct7_bits(w) {
                0x01 => Inst::REMW(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                _ => Inst::UNDEF(w),
            },
            0b111 => match funct7_bits(w) {
                0x01 => Inst::REMUW(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                _ => Inst::UNDEF(w),
            },
            _ => Inst::UNDEF(w),
        },
        0b11000 => match funct3_bits(w) {
            0b000 => Inst::BEQ(rs1_gpr(w), rs2_gpr(w), bimm_bits(w)),
            0b001 => Inst::BNE(rs1_gpr(w), rs2_gpr(w), bimm_bits(w)),
            0b100 => Inst::BLT(rs1_gpr(w), rs2_gpr(w), bimm_bits(w)),
            0b101 => Inst::BGE(rs1_gpr(w), rs2_gpr(w), bimm_bits(w)),
            0b110 => Inst::BLTU(rs1_gpr(w), rs2_gpr(w), bimm_bits(w)),
            0b111 => Inst::BGEU(rs1_gpr(w), rs2_gpr(w), bimm_bits(w)),
            _ => Inst::UNDEF(w),
        },
        0b11001 => Inst::JALR(rd_gpr(w), rs1_gpr(w), 0),
        0b11011 => Inst::JAL(rd_gpr(w), 0),
        0b11100 => match funct3_bits(w) {
            0b000 => match funct7_bits(w) {
                0x00 => Inst::ECALL,
                0x01 => Inst::EBREAK,
                0x08 => match (rs2_bits(w), rs1_bits(w), rd_bits(w)) {
                    (0b00010, 0b00000, 0b00000) => Inst::SRET,
                    _ => Inst::UNDEF(w),
                },
                0x18 => match (rs2_bits(w), rs1_bits(w), rd_bits(w)) {
                    (0b00010, 0b00000, 0b00000) => Inst::MRET,
                    _ => Inst::UNDEF(w),
                },
                _ => Inst::UNDEF(w),
            },
            0b001 => Inst::CSRRW(rd_gpr(w), rs1_gpr(w), csr_bits(w)),
            0b010 => Inst::CSRRS(rd_gpr(w), rs1_gpr(w), csr_bits(w)),
            0b011 => Inst::CSRRC(rd_gpr(w), rs1_gpr(w), csr_bits(w)),
            0b101 => Inst::CSRRWI(rd_gpr(w), uimm_bits(w), csr_bits(w)),
            0b110 => Inst::CSRRSI(rd_gpr(w), uimm_bits(w), csr_bits(w)),
            0b111 => Inst::CSRRCI(rd_gpr(w), uimm_bits(w), csr_bits(w)),
            _ => Inst::UNDEF(w),
        },
        _ => Inst::UNDEF(w),
    }
}

fn csr_bits(w: u32) -> Csr {
    Csr::from_u16(imm11_bits(w) as u16)
}

fn uimm_bits(w: u32) -> u32 {
    (w >> 15) & 0b11111
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

fn rd_bits(w: u32) -> u32 {
    (w >> 7) & 0b11111
}

fn rs1_bits(w: u32) -> u32 {
    (w >> 15) & 0b11111
}

fn rs2_bits(w: u32) -> u32 {
    (w >> 20) & 0b11111
}

fn rd_gpr(w: u32) -> Gpr {
    Gpr::from_u8(rd_bits(w) as u8).unwrap()
}

fn rs1_gpr(w: u32) -> Gpr {
    Gpr::from_u8(rs1_bits(w) as u8).unwrap()
}

fn rs2_gpr(w: u32) -> Gpr {
    Gpr::from_u8(rs2_bits(w) as u8).unwrap()
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
