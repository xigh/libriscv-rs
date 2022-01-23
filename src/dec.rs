use crate::{ByteReader, Csr, Gpr, Inst};

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
        }
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
            0b000 => Inst::LB(rd_gpr(w), rs1_gpr(w), imm12_bits(w)),
            0b001 => Inst::LH(rd_gpr(w), rs1_gpr(w), imm12_bits(w)),
            0b010 => Inst::LW(rd_gpr(w), rs1_gpr(w), imm12_bits(w)),
            0b011 => Inst::LD(rd_gpr(w), rs1_gpr(w), imm12_bits(w)),
            0b100 => Inst::LBU(rd_gpr(w), rs1_gpr(w), imm12_bits(w)),
            0b101 => Inst::LHU(rd_gpr(w), rs1_gpr(w), imm12_bits(w)),
            0b110 => Inst::LWU(rd_gpr(w), rs1_gpr(w), imm12_bits(w)),
            _ => Inst::UNDEF(w),
        },
        0b00011 => match funct3_bits(w) {
            0b000 => Inst::FENCE(rd_gpr(w), rs1_gpr(w), imm12_bits(w)),
            0b001 => Inst::FENCEI(rd_gpr(w), rs1_gpr(w), imm12_bits(w)),
            _ => Inst::UNDEF(w),
        },
        0b00100 => match funct3_bits(w) {
            0b000 => Inst::ADDI(rd_gpr(w), rs1_gpr(w), imm12_bits(w)),
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
            0b010 => Inst::SLTI(rd_gpr(w), rs1_gpr(w), imm12_bits(w)),
            0b011 => Inst::SLTUI(rd_gpr(w), rs1_gpr(w), imm12_bits(w)),
            0b100 => Inst::XORI(rd_gpr(w), rs1_gpr(w), imm12_bits(w)),
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
            0b110 => Inst::ORI(rd_gpr(w), rs1_gpr(w), imm12_bits(w)),
            0b111 => Inst::ANDI(rd_gpr(w), rs1_gpr(w), imm12_bits(w)),
            _ => unreachable!(),
        },
        0b00101 => Inst::AUIPC(rd_gpr(w), imm20_bits(w)),
        0b00110 => match funct3_bits(w) {
            0x00 => Inst::ADDIW(rd_gpr(w), rs1_gpr(w), imm12_bits(w)),
            0x01 => Inst::SLLIW(rd_gpr(w), rs1_gpr(w), shamt32_bits(w)),
            0x05 => match funct7_bits(w) {
                0x00 => Inst::SRLIW(rd_gpr(w), rs1_gpr(w), shamt32_bits(w)),
                0x20 => Inst::SRAIW(rd_gpr(w), rs1_gpr(w), shamt32_bits(w)),
                _ => Inst::UNDEF(w),
            },
            _ => Inst::UNDEF(w),
        },
        0b01000 => match funct3_bits(w) {
            0b000 => Inst::SB(rd_gpr(w), rs1_gpr(w), imm12_bits(w)),
            0b001 => Inst::SH(rd_gpr(w), rs1_gpr(w), imm12_bits(w)),
            0b010 => Inst::SW(rd_gpr(w), rs1_gpr(w), imm12_bits(w)),
            0b011 => Inst::SD(rd_gpr(w), rs1_gpr(w), imm12_bits(w)),
            _ => Inst::UNDEF(w),
        },
        0b01011 => match funct3_bits(w) {
            0b010 => match funct5_bits(w) {
                0b00010 => match rs2_bits(w) {
                    0b00000 => Inst::LRW(rd_gpr(w), rs1_gpr(w)),
                    _ => Inst::UNDEF(w),
                },
                0b00011 => Inst::SCW(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0b00001 => Inst::AMOSWAPW(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0b00000 => Inst::AMOADDW(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0b00100 => Inst::AMOXORW(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0b01100 => Inst::AMOANDW(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0b01000 => Inst::AMOORW(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0b10000 => Inst::AMOMINW(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0b10100 => Inst::AMOMAXW(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0b11000 => Inst::AMOMINUW(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0b11100 => Inst::AMOMAXUW(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                _ => Inst::UNDEF(w),
            },
            0b011 => match funct5_bits(w) {
                0b00010 => match rs2_bits(w) {
                    0b00000 => Inst::LRD(rd_gpr(w), rs1_gpr(w)),
                    _ => Inst::UNDEF(w),
                },
                0b00011 => Inst::SCD(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0b00001 => Inst::AMOSWAPD(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0b00000 => Inst::AMOADDD(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0b00100 => Inst::AMOXORD(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0b01100 => Inst::AMOANDD(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0b01000 => Inst::AMOORD(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0b10000 => Inst::AMOMIND(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0b10100 => Inst::AMOMAXD(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0b11000 => Inst::AMOMINUD(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                0b11100 => Inst::AMOMAXUD(rd_gpr(w), rs1_gpr(w), rs2_gpr(w)),
                _ => Inst::UNDEF(w),
            },
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
            0b000 => match (funct7_bits(w), rs2_bits(w), rs1_bits(w), rd_bits(w)) {
                (0b0000000, 0b00000, 0b00000, 0b00000) => Inst::ECALL,
                (0b0000000, 0b00001, 0b00000, 0b00000) => Inst::EBREAK,
                (0b0001000, 0b00010, 0b00000, 0b00000) => Inst::SRET,
                (0b0011000, 0b00010, 0b00000, 0b00000) => Inst::MRET,
                (0b0001000, 0b00101, 0b00000, 0b00000) => Inst::WFI,
                (0b0001001, _, _, 0b00000) => Inst::SFENCEVMA(rs1_gpr(w), rs2_gpr(w)),
                (0b0001011, _, _, 0b00000) => Inst::SINVALVMA(rs1_gpr(w), rs2_gpr(w)),
                (0b0001100, 0b00000, 0b00000, 0b00000) => Inst::SFENCEWINVAL,
                (0b0001100, 0b00001, 0b00000, 0b00000) => Inst::SFENCEINVALIR,
                (0b0010001, _, _, 0b00000) => Inst::HFENCEVVMA(rs1_gpr(w), rs2_gpr(w)),
                (0b0110001, _, _, 0b00000) => Inst::HFENCEGVMA(rs1_gpr(w), rs2_gpr(w)),
                (0b0010011, _, _, 0b00000) => Inst::HINVALVVMA(rs1_gpr(w), rs2_gpr(w)),
                (0b0110011, _, _, 0b00000) => Inst::HINVALGVMA(rs1_gpr(w), rs2_gpr(w)),
                _ => Inst::UNDEF(w),
            }
            0b100 => match (funct7_bits(w), rs2_bits(w), rs1_bits(w), rd_bits(w)) {
                (0b0110000, 0b00000, _, _) => Inst::HLVB(rd_gpr(w), rs1_gpr(w)),         
                (0b0110000, 0b00001, _, _) => Inst::HLVBU(rd_gpr(w), rs1_gpr(w)),        
                (0b0110010, 0b00000, _, _) => Inst::HLVH(rd_gpr(w), rs1_gpr(w)),         
                (0b0110010, 0b00001, _, _) => Inst::HLVHU(rd_gpr(w), rs1_gpr(w)),        
                (0b0110010, 0b00011, _, _) => Inst::HLVXHU(rd_gpr(w), rs1_gpr(w)),       
                (0b0110100, 0b00000, _, _) => Inst::HLVW(rd_gpr(w), rs1_gpr(w)),         
                (0b0110100, 0b00011, _, _) => Inst::HLVXWU(rd_gpr(w), rs1_gpr(w)),       
                (0b0110001, _, _, 0b00000) => Inst::HSVB(rs1_gpr(w), rs2_gpr(w)),
                (0b0110011, _, _, 0b00000) => Inst::HSVH(rs1_gpr(w), rs2_gpr(w)),
                (0b0110101, _, _, 0b00000) => Inst::HSVW(rs1_gpr(w), rs2_gpr(w)),
                (0b0110100, 0b00001, _, _) => Inst::HLVWU(rd_gpr(w), rs1_gpr(w)),
                (0b0110110, 0b00000, _, _) => Inst::HLVD(rd_gpr(w), rs1_gpr(w)),
                (0b0110111, _, _, 0b00000) => Inst::HSVD(rs1_gpr(w), rs2_gpr(w)),
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
    Csr::from_u16(imm12_bits(w) as u16)
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

fn funct5_bits(w: u32) -> u32 {
    (w >> 27) & 0b11111
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

fn imm12_bits(w: u32) -> u32 {
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
